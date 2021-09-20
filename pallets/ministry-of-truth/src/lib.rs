#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub use pallet_collective;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::{DispatchResult, EncodeLike},
		pallet_prelude::*,
	};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	use sp_runtime::traits::{AtLeast32BitUnsigned, CheckedAdd, One, Zero};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Id of Articles stored in the system
		type ArticleId: Parameter + Member + AtLeast32BitUnsigned + Default + Copy;
	}

	/// Id of claims made in the system. 
	type ClaimId = u32;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[derive(Encode, Decode, Default, Clone, Eq, PartialEq, RuntimeDebug)]
	pub struct ProposedArticle {
		// author: AuthorId
		/// The URL designated for accessing the Article text content.
		url: Vec<u8>,
		/// Ids of claims raised in the article
		claims: Vec<u32>,
		/// The originating id of the source
		source_id: Vec<u8>,
	}

	#[pallet::storage]
	#[pallet::getter(fn get_article)]
	pub type ArticleStorage<T: Config> =
		StorageMap<_, Blake2_128Concat, T::ArticleId, ProposedArticle, ValueQuery>;

	/// Next available class ID.
	#[pallet::storage]
	#[pallet::getter(fn next_class_id)]
	pub type NextArticleId<T: Config> = StorageValue<_, T::ArticleId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn next_claim_id)]
	pub type NextClaimId<T: Config> = StorageValue<_, ClaimId, ValueQuery>;

	#[derive(Encode, Decode, Default, Clone, Eq, PartialEq, RuntimeDebug)]
	pub struct Claim {
		/// the IPFS CID of the text that contains the objective claim statement.
		pub claim_text_cid: Vec<u8>,
		/// Whether the claim is determined to be credible or non-credible. We'll use a negative for the boolean due to science's focus on falsifying ideas.
		pub is_rejected: bool,
	}

	#[pallet::storage]
	#[pallet::getter(fn get_claims)]
	/// Double Storage map to map claims to the articles they were discovered on
	pub type ClaimsToArticles<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		ClaimId,
		Blake2_128Concat,
		T::ArticleId,
		Claim,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ArticleStored(T::ArticleId),
		ClaimStored(ClaimId),
		CouldNotRetrieveArticle
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NoAvailableArticleId,
		NoAvailableClaimId,
		NonExistentArticle,
		Unknown,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		/// Stores an article in the system for voting. 
		///
		/// # Arguments
		///
		/// * `origin` - Origin of the request
		/// * `source_id` - Unique identifier, or DOI of the article
		/// * `url` - Url of the article. Displayed for the purpose of allowing voters to find and read the content.
		pub fn store_article(
			origin: OriginFor<T>,
			source_id: Vec<u8>,
			url: Vec<u8>,
		) -> DispatchResult {
			ensure_signed(origin)?;
			// get id
			let class_id =
				NextArticleId::<T>::try_mutate(|id| -> Result<T::ArticleId, DispatchError> {
					let current_id = *id;
					*id = id.checked_add(&One::one()).ok_or(Error::<T>::NoAvailableArticleId)?;
					Ok(current_id)
				})?;

			let article = ProposedArticle { url, claims: [].to_vec(), source_id };
			ArticleStorage::<T>::insert(class_id.clone(), article);
			Self::deposit_event(Event::ArticleStored(class_id));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		/// Stores a claim for a given article. This should be called by the collective propose extrinsic
		///
		/// # Arguments
		///
		/// * `origin` - Origin of the request
		/// * `claim_statement` - IPFS CID of a stored string that contains an objective claim. This claim will be voted on for veracity.
		/// * `article_id` - Id of an article which contained the given claim
		/// * `is_rejected` - Designates whether the claim is non-credible, or credible
		pub fn store_claim_for_article(
			origin: OriginFor<T>,
			claim_statement: Vec<u8>,
			article_id: T::ArticleId,
			is_rejected: bool,
		) -> DispatchResult {
			// TODO: Find way to ensure this was called by the `propose` extrinsic.
			ensure_signed(origin)?;
			// Ensure that the article exists
			ensure!(ArticleStorage::<T>::contains_key(article_id), Error::<T>::NonExistentArticle);

			let new_claim_id =
				NextClaimId::<T>::try_mutate(|claim_id| -> Result<ClaimId, DispatchError> {
					let current_id = *claim_id;
					*claim_id =
						claim_id.checked_add(One::one()).ok_or(Error::<T>::NoAvailableClaimId)?;
					Ok(current_id)
				})?;

			ClaimsToArticles::<T>::insert(
				new_claim_id,
				article_id.clone(),
				Claim { claim_text_cid: claim_statement, is_rejected },
			);

			ArticleStorage::<T>::try_mutate_exists(article_id.clone(), |val| -> DispatchResult {
				// add claim id to article for future reference
				let article = val
					.as_mut()
					.ok_or(Error::<T>::NonExistentArticle)
					.unwrap();

				article.claims.push(new_claim_id);
				Self::deposit_event(Event::ClaimStored(new_claim_id));
				Ok(())
			});

			Self::deposit_event(Event::ClaimStored(new_claim_id));
			Ok(())
		}
	}
}
