
// The validator component will be responsible for validating the secrets
// It will check if the secret is expired or not
// There it needs to check the following flow
// - Is a secret ignored?
//   YES:
//   - Return not expired
//   NO: -
// - Does the secret use a name pattern?
//   YES:
//   - What retentation in days is used in the pattern? Store it as retention time
//   NO: -
// - Calculate diff days and check if it is greater than the retention time. Use default retention time if not set
//   YES:
//   - Return expired
//   NO:
//   - Return not expired

pub struct Validator;