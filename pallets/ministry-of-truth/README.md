# Ministry of Truth 🏢 #

### A pallet for private blockchains focused on interpretability and consensus of claims made in the scientific community, and misinformation solution foundations. ###

#### What is this? ####
This pallet attempts to implement the consensus solution described in the paper [*The use of distributed consensus algorithms to curtail the spread of medical misinformation*](https://www.ijam-web.org/article.asp?issn=2455-5568;year=2019;volume=5;issue=2;spage=93;epage=99;aulast=Plaza). The paper lays out a hypothetical case for a private blockchain solution to reduce groupthink, and improve the consensus-gathering of the scientific process, as well as improve the interpretability of the scientific results. The authors of the paper give several examples of cases where such a system may have countered the spread of medical misinformation. One example is the anti-vaccination movement in the 1990's following the publication of a paper which was widely regarded as faulty by the scientific community.

#### What does the name mean? ####
The name of the pallet is inspired by the [propaganda arm of the government in the science-fiction novel *1984*](https://en.wikipedia.org/wiki/Ministries_of_Nineteen_Eighty-Four#Ministry_of_Truth), that serves to determine what information is true. The name is just a silly reference and has nothing to do with the function of the pallet.

#### Working with Collective ####
This pallet is written with the understanding that it lives alongside the [`Collective` Pallet](https://substrate.dev/rustdocs/latest/pallet_collective/index.html). It is expected that the extrinsics in the MOT pallet would be proposed through the `Collective propose` extrinsic, and not run directly by users. 

#### Context for the System ####
The pallet should follow the author's guidance regarding behavior and voting. It should facilitate steps in the scientific process. In this case, the step to facilitate is a post-publish peer review step. The author recommends in the paper that a private blockchain consisting of anonymized node operators who are members of the scientific community be implemented. Ostensibly, this is to reduce groupthink in voting. A similar reduction in bias might be achieved if the pallet is split into multiple voting steps, and different instances of `Collective` consisting of randomized participants are used in voting. 

#### How would it be used? ####
 The usage of the system can be described through the following flow:

1. The user enters the app UI (imagine in this case there exist a UI form specifically for this pallet).
2. The user enters their article url into the form and submits it. The DOI is retrieved by the UI code or entered by the user.
3. The UI sends a request to the node, requesting that the pallet store the article using the `store_article` extrinsic.
4. This stores the article in a StorageMap, `ArticleStorage`, which is designated for content that is in MOT's peer-review process . The `claims` vec of this struct is initialized as empty.
5. The content will now be shown in the UI, under a peer review page, along with any other content in the same `StorageMap`.
6. These members can now participate in the claims-voting step in the process. They can identify an objective claim statement for a claim made by the article, and put it to vote by using the Claims UI. This part of the UI contains a form that raises a motion in the *Collective* pallet that proposes calling the MOT's *store_claim_for_article* extrinsic with: their objective claim statement, the article ID the claim was discovered in, and a boolean value indicating whether the claim is accepted/rejected as true or false.
7. Other members can vote aye/nay on such claims. Aye = accepted objective claim. Nay = non-credible OR subjective claim. In the future, claims will further be split into two steps: 1. determining whether claims are objective, and 2. determining whether claims are true. This can be further split between different instances of collective, with randomized members.
8. Following the close of a voting period for claims on an article, a score is given to the article, based on the ratio of accepted/rejected claims.


#### Goals #### 
1. Provide a decentralized tool for improving scientific community consensus
2. Allow for scientific community member verification and anonymity
3. Provide a mechanism for agreeing on public-facing study results
4. Provide a reference point for first-degree sources of information
5. Provide a foundation for developing other misinformation solutions based on non-first degress information sources
6. Provide a database of publicly-agreed, disproven claims
## Interacting with the Pallet ##
1. Build the node `cargo build --release`
2. Run the dev chain locally `./target/release/node-template --dev --tmp`
3. Interact with the node via the polkadotjs UI:
	1. Use the `store_article` extrinsic
		1. Go to https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer
		2. Choose the `storeArticle` extrinsic of the `ministryOfTruth` pallet. (In real-world use, the `Collective` pallet `propose` extrinsic would call this extrinsic)
		3. Provide hex values for the article url and source id (DOI) for the article
		4. Submit the transaction
	2. Use the `store_claim_for_article` extrinsic
		1. Choose the `storeClaimForArticle` extrinsic of the `ministryOfTruth` pallet.(In real-world use, users would see proposed articles and would suggest and suggest claims for the article through the `propose` extrinsic of the `Collective` pallet)
		2. Provide hex values for the claim statement(objective text of the claim that the user has identified), article id(refers to an article id you got from the previous step), and `isRejected`(whether the claim is credible, or non-credible.)
		3. Submit the transaction
	Note: In a real-world case, this is where users would gather and vote on the motions of whether to store the claims as "accepted" objective claims.

#### Fixes/Improvements/TODO ####
1. Add extrinsics that call the `Collective` `propose` extrinsic with the `Call` of the expected article/claim contents. This would make the experience more guided and rely less on the frontend code to provide an exact call to the Collective extrinsics. 
2. Fix issue where the Polkadotjs UI can't retrieve `claimsToArticles` (It works in tests, but getting a strange error in the apps UI).
3. Fix BadOrigin error when using `ministry-of-truth` extrinsics from the `Collective` `propose` extrinsic.
4. Split claims extrinsic into two: one for voting on whether such claim was made in an article, and another for voting on claim veracity of verified objective claims. It would likely reduce bias if multiple different groups could vote on a. The claims made in the article, and b. whether those claims are accepted/rejected.
5. Add additional collective instances. Add code to randomize members of collective instances to ensure roles are rotated. 
6. Assign score to articles based on accepted/rejected claims on the article.
7. Store an `article` vec on `Claims` to provide a many-to-many relationship of claims to articles. This would make more sense than the current relationship as one claim can appear in multiple articles. A step to de-dupe claims by users would need to be implemented with such a relationship.
8. Provide an additional field to denote a news-facing claim. An easily understandable, agreed-upon, outward-facing claim containing limited jargon to help communicate results to media or laymen readers. This is essentially an agreed-upon interpretation of the results.

#### Why might this be useful? ####
In addition to the improvements noted by the author, a solution focused on accumulating some amount of primary information on-chain could serve as a foundation for solutions to other issues in misinformation and identity. For example, a solution similar to the one posed here, though focused on second or third degree sources of information such as news articles and tweets could implement some sort of reference system based on the first degree sources of information. Credibility can be tracked according to author, and or publisher. 