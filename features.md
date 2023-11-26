# Project Overview

The main objective of this project is to build a decentralized rewards platform
that provides an innovative and flexible way for businesses to run promotional
campaigns and loyalty programs.

## Essential Features (MVP)

**Core Functionality**

  * Business campaign creation and configuration
  * Reward token minting and allocation
  * User dashboard for tracking reward progress
  * Token burning/redemption for rewards

**Blockchain Integration**

  * Crypto wallet connectivity (Metamask)
  * Campaign funding via crypto
  * NFT generation and claiming

**Simplified Areas**

  * Focus on a single blockchain network integration (Ethereum)
  * Start with predefined branded template campaigns
  * Fixed reward redemption catalogs vs dynamic listings
  * Basic analytics and reporting

### Simplified UI/UX

  * Clean, minimalist interfaces focused on core actions
  * Intuitive navigation and flow for wallet connectivity
  * Tooltips and help guides for onboarding
  * Visual cues to signal blockchain interactions
  * Notification alerts for key events

**Blockchain Functionalities**

  * Seamless wallet connectivity
    * MetaMask Integration
      * Auto-detection of MetaMask plugin installation
      * Clear call-to-action to connect wallet if not found
      * Support testnet wallets during development
      * Handle scenario when user switches accounts
      * Auto reload on chain changes to sync status
      * Alert user when switched to incorrect network

  * Payment via Cryptocurrencies
    * Allow funding campaigns with stablecoins (USDC, DAI, BUSD)
    * Manage balance transfers
    * Allow topping up balance via MetaMask
    * Dynamic exchange rates from Oracle provider
    * Payment summaries before confirmations
    * Configurable thresholds by admin

  * Payment Threshold Config
    * Default global thresholds across platform (min/max limits)
    * Admin configurable per campaign
      * Custom minimum payment amount
      * Capping flow for high-value rewards
    * Limit use cases
      * Avoid trivial transactions
      * Manage flow for high-value rewards
    * Analytics on payment patterns
    * Policy fine-tuning based on usage

  * Gas Fee Transparency
    * Estimates of gas costs when submitting transactions
    * Option for users to edit gas price (slow/fast/rapid)
    * View current Ethereum network gas price trends
    * Alerts if gas fee is exceptionally high
    * Retry mechanism if transaction fails due to surge
    * Notifications on successful or failed attempts
    * Support setting gas limits where feasible

  * NFT Integration
    * Templates for simple NTF Creation
      * Start with 3-4 base templates showcasing campaign branding
      * Text, images and graphic elements editable
      * Background theme color variants
      * Render NFT image previews before minting
      * Appropriate metadata standards like ERC-721
    * Claiming and viewing by users

### Key Functionalities

**Capaign Budget Funding**

  * Payment integration to accept fiat and crypto funding from businesses
  * Mint equivalent reward tokens to campaign pool based on funding
_Considerations_:
  * Support payment methods like card, bank transfer ect.
  * Manage floating fiat-crypto conversions

**Reward Distribution**

  * Logic to distribute rewards to users based on campaign criteria
  * Transfer of tokens and NFTs to users wallets
_Consideration_:
  * Applying custom business logic for reward releases
  * Gas optimization for token transfers

**Token Burning**

  * Burning tokens upon redemption to control circulation
  * Ensuring token supply management as per monetary policy
_Considerations_:
  * Security reviews to prevent token loss or locking
  * Analytics on burn rate and demand modeling

**Access Control**

  * Administrator functions to manage registrations
  * Portal for monitoring campaigns and transactions
_Considerations_:
  * Prevent privilege escalation or unauthorized access
  * Implement multi-signature workflows

### Custom Business Rules for Reward Releases

1. Activity threshold
   * Requiring users to complete certain actions to trigger rewards -
     e.g. 5 social media posts to earn 100 tokens

2. Schedule-based Unlocks
   * Making rewards available only on certain dates or times to drive engagement

3. Randomized Rewards
   * Adding element of surprise with variable reward quantities and values

4. Referral Bonuses
   * Added rewards for new user signups through member referrals

5. Loyalty Tiers
   * Increased rewards for members reaching higher loyalty statuses

6. Real-world Triggers
   * Geo-location check-ins releasing localized rewards

_In terms of access controls and admin permissions_:
* View-only Analyst - Can see campaigns data, run reports
* Campaign Manager - Configure campaigns, allocate rewards
* Finance Admin - Fund/withdraw balance oversight
* Application Admin- Manage security, upgrades, new feature enablement

### Transparency of Business Rules

* All active campaign rules published transparently on the platform for users
* Changes to campaign rules reflected in real-time with notifications
* User dashboard clearly showing reward criteria and progress
* Smart contracts emit events for major rule modifications
* Frontend captures events and reflects changes
* Version tracking for user agreement on rules

### Traceability

* Campaign smart contracts and rules published openly for inspection
* Participation agreements capture user consent to rules
* Blockchain transactions provide immutable execution proof
* Third party auditing support for unbiased verification

### Smart Contract Upgrades

* Take modular approach allowing piece-wise upgrades
* New functionality deployed in separate contracts
* Interfaces used for backward compatibility
* Proxy contracts to redirect transactions across versions
* Deprecate older functionality gradually after testing

### JavaScript/React and Smart Contracts Implementation

**Frontend with ReactJS:**

  * _Modular Components_ - Break UI into reusable encapsulated components for
    maintainability.
  * _Declarative Nature_ - Component logic defined along with output UI
    declaratively
  * _One-way Data Flow_ - Follow a structured unidirectional data flow
    architecture
  * _React Hooks_ - Use built-in hooks for state and effects over classes
  * _Responsive Design_ - Mobile-first adaptive UI implementation
  * _TypeSafety_ - Use TypeScript for adding types to JavaScript
  * _Error Handling_ - Implement consistent centralized error handling

**Smart Contracts:**

  * _Security Focus_ - Follow best practices like avoiding loops, overflows
  * _Modular Design_ - Break contracts into importable interfaces and libraries
  * _Readability_ - Favor descriptive names and separation of concerns
  * _Validation_ - All external inputs validated before processing
  * _Testing Suite_ - Build extensive unit test coverage to prevent issues
  * _Gas Optimization_ - Architect for lower fees by avoiding redundant operations
  * _Events and Logging_ - Incorporate events for transaction life cycle tracking
  * _Upgradeability_ - Support upgrading contract logic while preserving data

### Dashboard Functionality

**User Dashboard:**

  * Display active campaigns user has joined
  * Overview of total rewards collected and redeemed
  * _Per Campaign_:
    * Rewards rules and criteria
    * Counters showing tokens accumulated
    * Progress trackers / checklists reflecting completion status
    * Timelines/deadlines for scheduled unlocks

**Communication for Upgrades**:
  * Email newsletter to active users on upcoming upgrades
  * In-app notifications for urgent or mandatory upgrades
  * Support articles detailing changes for public reference
  * User access to historical contract versions
  * "What's New" page documenting latest features
  * Feedback module to capture user comments
  * Townhall discussions to address concerns


    * Notifications feed on new rewards and bonuses unlocked
