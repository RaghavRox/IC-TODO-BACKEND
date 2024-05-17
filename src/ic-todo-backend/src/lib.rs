use std::{cell::RefCell, collections::BTreeMap};

thread_local! {
    static ID_COUNTER: RefCell<candid::Nat> = RefCell::new(candid::Nat::from(0u8));
    static STORE: RefCell<BTreeMap<candid::Nat, String>> = const { RefCell::new(BTreeMap::new()) };
}

#[ic_cdk::update]
fn post(todo: String) -> String {
    ID_COUNTER.with(|id_counter| {
        STORE.with_borrow_mut(|store| {
            store.insert(id_counter.borrow().to_owned(), todo);
        });
        let id_string = id_counter.borrow().to_string();
        *id_counter.borrow_mut() += 1u64;
        id_string
    })
}

#[ic_cdk::update]
fn update(id: candid::Nat, todo: String) -> Option<String> {
    STORE.with_borrow_mut(|store| {
        store.get(&id)?;
        store.insert(id.to_owned(), todo)
    })
}

#[ic_cdk::update]
fn delete(id: candid::Nat) -> Option<String> {
    STORE.with_borrow_mut(|store| store.remove(&id))
}

#[ic_cdk::query]
fn get(id: candid::Nat) -> Option<String> {
    STORE.with_borrow(|store| store.get(&id).cloned())
}

#[ic_cdk::query]
fn get_with_pagination(limit: usize, offset: usize) -> Vec<(candid::Nat, String)> {
    STORE.with_borrow(|store| {
        store
            .iter()
            .skip(offset)
            .take(limit)
            .map(|(id, todo)| (id.to_owned(), todo.to_owned()))
            .collect()
    })
}

ic_cdk::export_candid!();
