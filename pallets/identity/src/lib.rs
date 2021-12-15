#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::{DispatchResult, DispatchResultWithPostInfo},
		pallet_prelude::*,
		sp_runtime::traits::{Hash, Zero},
		traits::{Currency, ExistenceRequirement, Randomness},
		transactional, inherent,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use frame_support::inherent::Vec;

	use pallet_kitties::{
		Error as KittiesError, Kitties, Pallet as PalletKitties, Kitty
	};
	
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_kitties::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Assigned(T::Hash, T::Hash),
		ReturnedKitty(T::Hash)
	}

	#[pallet::storage]
	#[pallet::getter(fn kitties_identities)]
	pub(super) type KittiesIdentities<T: Config> = StorageMap<_, Twox64Concat, T::Hash, T::Hash>;

	#[pallet::error]
	pub enum Error<T> {
		NoIdentityAssociated,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(100)]
		pub fn assign_identity(
			origin: OriginFor<T>,
			identity: Vec<u8>,
			kitty_id: T::Hash,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;

			ensure!(PalletKitties::<T>::is_kitty_owner(&kitty_id, &from)?, <KittiesError<T>>::NotKittyOwner);

			ensure!(<Kitties<T>>::contains_key(&kitty_id), <KittiesError<T>>::KittyNotExist);

			let identity_hash = T::Hashing::hash_of(&identity);

			<KittiesIdentities<T>>::insert(&identity_hash, &kitty_id);

			Self::deposit_event(Event::Assigned(identity_hash.clone(), kitty_id.clone()));

			Ok(())
		}

		#[pallet::weight(100)]
		pub fn get_kitty_by_identity(origin: OriginFor<T>, identity: Vec<u8>) -> DispatchResult {
			let from = ensure_signed(origin)?;

			let kitty_id = Self::kitties_identities(T::Hashing::hash_of(&identity)).ok_or(<Error<T>>::NoIdentityAssociated)?;

			ensure!(PalletKitties::<T>::is_kitty_owner(&kitty_id, &from)?, <KittiesError<T>>::NotKittyOwner);

			let kitty = <Kitties<T>>::get(&kitty_id).unwrap();

			Self::deposit_event(Event::ReturnedKitty(kitty_id.clone()));
			
			Ok(())
		}
	}
}
