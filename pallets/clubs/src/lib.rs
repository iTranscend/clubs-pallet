#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
pub mod weights;

pub use pallet::*;
pub use weights::WeightInfo;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use crate::WeightInfo;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::{vec::Vec};
	use sp_std::default::Default as OtherDefault;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type MininmumClubs: Get<u8>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emmitted when a member has been added to a club
		MemberAdded(Vec<u8>, T::AccountId),
		/// Event emmitted when a member has been removed from a club
		MemberRemoved(Vec<u8>, T::AccountId),
	}

	#[pallet::storage]
	#[pallet::getter(fn clubs)]
	/// Maps each Club to its Member Accounts
	pub(super) type Clubs<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<T::AccountId>, OptionQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub phantom: PhantomData<T>,
		pub clubs: Option<Vec<(Vec<u8>, Vec<T::AccountId>)>>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> OtherDefault for GenesisConfig<T> {
		fn default() -> Self {
			Self { phantom: OtherDefault::default(), clubs: None }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			if let Some(clubs) = &self.clubs {
				for (idx, club) in clubs {
					Clubs::<T>::insert(idx, club);
				}
			}
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Club is not minimum.
		NumberOfClubsBelowMin,
		/// Club does not exist
		ClubDoesNotExist,
		/// Member already exists in club
		MemberAlreadyExistsInClub,
		/// Member does not exist in club
		MemberDoesNotExistInClub,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Adds a member to a club and emits an event
		#[pallet::weight(<T as Config>::WeightInfo::add_member())]
		pub fn add_member(
			origin: OriginFor<T>,
			club: Vec<u8>,
			member: T::AccountId,
		) -> DispatchResult {
			// Make sure the caller is root
			ensure_root(origin)?;

			// Ensure club exists in storage
			ensure!(Clubs::<T>::contains_key(club.clone()) == true, Error::<T>::ClubDoesNotExist);

			// Ensure member does not already exist
			let mut existing_members = Clubs::<T>::get(club.clone()).unwrap();
			ensure!(!existing_members.contains(&member), Error::<T>::MemberAlreadyExistsInClub);

			existing_members.push(member.clone());

			Clubs::<T>::try_mutate(club.clone(), |members| -> DispatchResult {
				*members = Some(existing_members);
				Ok(())
			})?;

			Self::deposit_event(Event::MemberAdded(club.clone(), member.clone()));

			Ok(())
		}

		// Removes a member from a club and emits an event if successful
		#[pallet::weight(<T as Config>::WeightInfo::remove_member())]
		pub fn remove_member(
			origin: OriginFor<T>,
			club: Vec<u8>,
			member: T::AccountId,
		) -> DispatchResult {
			// Make sure the caller is root
			ensure_root(origin)?;

			// Ensure club exists in storage
			ensure!(Clubs::<T>::contains_key(club.clone()) == true, Error::<T>::ClubDoesNotExist);

			// Ensure member already exists
			let mut existing_members = Clubs::<T>::get(club.clone()).unwrap();
			ensure!(existing_members.contains(&member), Error::<T>::MemberDoesNotExistInClub);

			let member_index = existing_members.iter().position(|x| *x == member.clone()).unwrap();
			existing_members.remove(member_index);

			Clubs::<T>::try_mutate(club.clone(), |members| -> DispatchResult {
				*members = Some(existing_members);
				Ok(())
			})?;

			Self::deposit_event(Event::MemberRemoved(club.clone(), member.clone()));

			Ok(())
		}
	}
}
