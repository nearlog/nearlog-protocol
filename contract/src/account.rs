use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Account {
    /// A copy of an account ID. Saves one storage_read when iterating on accounts.
    pub account_id: AccountId,
    /// A list of assets that are supplied by the account.
    /// It's not returned for account pagination.
    pub supplied: HashMap<TokenId, Shares>,
    /// A list of option orders are bought by the account
    pub options: HashMap<OptionId, OptionOrder>,
    /// Tracks changes in storage usage by persistent collections in this account.
    #[borsh_skip]
    #[serde(skip)]
    pub storage_tracker: StorageTracker,
}

impl Account {
    pub fn new(account_id: &AccountId) -> Self {
        Self {
            account_id: account_id.clone(),
            supplied: HashMap::new(),
            options: HashMap::new(),
            storage_tracker: Default::default(),
        }
    }
}

impl Contract {
    pub fn internal_get_account(&self, account_id: &AccountId) -> Option<Account> {
        self.accounts.get(account_id).map(|o| o.into())
    }

    pub fn internal_unwrap_account(&self, account_id: &AccountId) -> Account {
        self.internal_get_account(account_id)
            .expect("Account is not registered")
    }

    pub fn internal_set_account(&mut self, account_id: &AccountId, mut account: Account) {
        let mut storage = self.internal_unwrap_storage(account_id);
        storage
            .storage_tracker
            .consume(&mut account.storage_tracker);
        storage.storage_tracker.start();
        self.accounts.insert(account_id, &account.into());
        storage.storage_tracker.stop();
        self.internal_set_storage(account_id, storage);
    }
}

#[near_bindgen]
impl Contract {
    /// Returns detailed information about an account for a given account_id.
    /// The information includes all supplied assets.
    /// Each asset includes the current balance and the number of shares.
    pub fn get_account(&self, account_id: AccountId) -> Option<Account> {
        self.internal_get_account(&account_id)
    }

    /// Returns the number of accounts
    pub fn get_num_accounts(&self) -> u32 {
        self.accounts.len() as _
    }
}
