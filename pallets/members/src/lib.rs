#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;



#[frame_support::pallet]
pub mod pallet {
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

/// Configure the pallet by specifying the parameters and types on which it depends.
#[pallet::config]
pub trait Config: frame_system::Config {
  /// Because this pallet emits events, it depends on the runtime's definition of an event.
  type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
}


// Pallets use events to inform users when important changes are made.
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
  /// Event emitted when a new member is added.
  NewMemberAdded { member: T::AccountId },
  /// Event emitted when a member is removed.
  MemberRemoved { member: T::AccountId },
}


#[pallet::error]
pub enum Error<T> {
  /// The member already exists.
  AlreadyAdded,
  /// Member does not exist so can not be removed
  NoSuchMember,
}


#[pallet::storage]
#[pallet::getter(fn members)]
/// Keeps track of members.
pub(super) type Members<T: Config> = StorageMap< _, Twox64Concat, T::AccountId, u8, ValueQuery,>;

// Dispatchable functions allow users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
#[pallet::call]
impl<T: Config> Pallet<T> {

  #[pallet::weight(0)]
  pub fn add_member(origin: OriginFor<T>, member: T::AccountId) -> DispatchResult {
    // Check that the extrinsic was signed and get the signer.
    // This function will return an error if the extrinsic is not signed.
    // let sender = ensure_signed(origin)?;
    // // check if the user hat root privileges 
    ensure_root(origin)?;

    // Verify that the specified member has not already been added.
    ensure!(!Members::<T>::contains_key(&member), Error::<T>::AlreadyAdded);

    // Add the member .
    Members::<T>::insert(&member, 1);

    // Emit an event that a new member has been added.
    Self::deposit_event(Event::NewMemberAdded { member: member });

    Ok(())
  }

  #[pallet::weight(0)]
  pub fn remove_member(origin: OriginFor<T>, member: T::AccountId) -> DispatchResult {
    // Check that the extrinsic was signed and get the signer.
    // This function will return an error if the extrinsic is not signed.
    // let sender = ensure_signed(origin)?;
    // // check if the user hat root privileges 
    ensure_root(origin)?;

    // Verify that the specified member exists.
    ensure!(Members::<T>::contains_key(&member), Error::<T>::NoSuchMember);

    // Remove the member .
    Members::<T>::remove(&member);

    // Emit an event that a member has been removed.
    Self::deposit_event(Event::MemberRemoved { member: member });

    Ok(())
  }

  // #[pallet::weight(0)]
  // pub fn create_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
  //   // Check that the extrinsic was signed and get the signer.
  //   // This function will return an error if the extrinsic is not signed.
  //   let sender = ensure_signed(origin)?;

  //   // Verify that the specified claim has not already been stored.
  //   ensure!(!Claims::<T>::contains_key(&claim), Error::<T>::AlreadyClaimed);

  //   // Add the member .
  //   Members::<T>::insert(&member, 1);

  //   // Emit an event that the claim was created.
  //   Self::deposit_event(Event::NewMemberAdded { member: sender, claim });

  //   Ok(())
  // }

  // #[pallet::weight(0)]
  // pub fn revoke_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
  //   // Check that the extrinsic was signed and get the signer.
  //   // This function will return an error if the extrinsic is not signed.
  //   let sender = ensure_signed(origin)?;

  //   // Get owner of the claim, if none return an error.
  //   let (owner, _) = Claims::<T>::get(&claim).ok_or(Error::<T>::NoSuchClaim)?;

  //   // Verify that sender of the current call is the claim owner.
  //   ensure!(sender == owner, Error::<T>::NotClaimOwner);

  //   // Remove claim from storage.
  //   Claims::<T>::remove(&claim);

  //   // Emit an event that the claim was erased.
  //   Self::deposit_event(Event::ClaimRevoked { who: sender, claim });
  //   Ok(())
  // }
}
}