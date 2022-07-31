use std::{
    marker::PhantomData,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

// 使用泛型来区分不同的类型
pub struct Customer<T> {
    id: u64,
    name: String,
    // 使用 phantomData 占位
    _type: PhantomData<T>,
}

pub trait Free {
    fn feature1(&self);
    fn feature2(&self);
}

pub trait Personal: Free {
    fn advance_feature(&self);
}

/// 为任何 Customer<T> 类型实现 Free  trait
impl<T> Free for Customer<T> {
    fn feature1(&self) {
        println!("feature 1 for {}", self.name);
    }

    fn feature2(&self) {
        println!("feature 2 for {}", self.name);
    }
}

/// 只为 Customer<PersonalPlan> 类型实现 Personal trait
/// Customer<PersonalPlan> 和 Customer<FreePlan> 不是同一种类型！
impl Personal for Customer<PersonalPlan> {
    fn advance_feature(&self) {
        println!("dear {}.", self.name);
    }
}

pub struct FreePlan;
pub struct PersonalPlan(f32);

impl<T> Customer<T> {
    pub fn new(name: String) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name,
            _type: PhantomData::default(),
        }
    }
}

impl From<Customer<FreePlan>> for Customer<PersonalPlan> {
    fn from(c: Customer<FreePlan>) -> Self {
        Self::new(c.name)
    }
}

pub fn subscribe(customer: Customer<FreePlan>, payment: f32) -> Customer<PersonalPlan> {
    let _plan = PersonalPlan(payment);
    customer.into()
}

fn main() {
    let customer = Customer::<FreePlan>::new("tyr".into());
    customer.feature1();
    customer.feature2();
    /// Customer<FreePlan> 没有实现 Personal trait，不能执行 advance_feature 方法
    // customer.advance_feature();
    let customer = subscribe(customer, 1.22);
    customer.feature1();
    customer.feature2();
    customer.advance_feature();
}