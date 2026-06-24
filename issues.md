#78 feat(frontend): build user settings page with wallet and notification preferences

Description
Users need a centralized settings page to manage their connected wallets, view verified remittance addresses, update email/webhook endpoints for notifications, and configure contractor details (if registered as a supplier).

Branch: feat/user-settings

Example commits:

feat(frontend): create user settings layout with tabs
feat(frontend): implement notification preference toggles
feat(frontend): add verified wallets management panel
Scope & Tasks
Settings Route (src/app/settings/page.tsx): Create the page with tabs:
Profile: Basic info, linked email address.
Wallets: View connected Stellar address, list of verified non-Stellar remittance sending wallets.
Notifications: Email toggles (Payment due, Milestone updates, Loan approval) and partner webhook URL configuration.
Developer/Contractor: View contractor whitelist status, whitelisted registration details.
Form Management: Save settings via backend database endpoints (POST /api/user/settings).
Validation: Validate email format and webhook URL accessibility.
Acceptance Criteria
 Settings page renders at /settings with tab navigation
 Email toggles and webhook URL fields save choices successfully
 Connected/verified wallets are listed with chain type badges
 Webhook URL validation checks for correct format



#81 feat(frontend): build multi-signature key coordinator interface

Description
The architecture leverages Stellar's native multisig capability for milestone governance. To make setting up a multisig governance account user-friendly, the frontend needs a multisig coordinator screen. This guides committee members through registering keys, setting thresholds, and building the setup transaction.

Branch: feat/multisig-coordinator

Example commits:

feat(frontend): create multisig setup wizard page
feat(frontend): build key weight and threshold adjustment inputs
feat(frontend): implement multisig setup transaction builder
Scope & Tasks
Multisig Page (src/app/multisig/page.tsx):
Input list for committee member public keys.
Input weights per key and threshold limits (Low, Medium, High threshold).
Transaction Builder:
Build a transaction containing SetOptions operations (adjusting signers, weights, and thresholds) for the governance account.
Display a visual representation of key distributions.
Freighter Signing: Sign the transaction using the master key of the governance account to apply parameters on-chain.
Validation: Ensure sum of key weights meets or exceeds the set thresholds.
Acceptance Criteria
 Interface allows adding public keys with weights
 Master account can set options via Freighter transaction
 Weight configuration validation prevents lockouts (sum of weights >= threshold)
 Page is responsive and clearly explains Stellar multisig mechanics