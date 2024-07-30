use std::{cell::RefCell, collections::HashMap};

use candid::Principal;
use ic_cdk::caller;

thread_local! {
    static CHAT: RefCell<HashMap<[Principal; 2], Vec<String>>> = RefCell::default();

}


#[ic_cdk::query]
fn get_chat(user_path: [Principal; 2]) -> Option<Vec<String>> {
    CHAT.with_borrow(|chats|  chats.get(&user_path).cloned())
}

#[ic_cdk::update]
fn add_chat_msg(msg: String, user2: Principal) {
    let user1 = caller();

    if user1 == Principal::anonymous() {
        panic!("Anonymous Principal!")
    }

    let mut principals = [user1, user2];
    principals.sort();

    CHAT.with_borrow_mut(|chats| {
        let mut_chat = chats.get_mut(&principals);

        if let Some(chat_vec) = mut_chat {
            chat_vec.push(msg);
        } else {
            chats.insert(principals, vec![msg]);
        }
    })
}
