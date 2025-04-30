use crate::semiring::Addition;
use crate::{date, Bool, Date, HashMap, Record, VarChar, FALSE};
use once_cell::sync::Lazy;
use ordered_float::OrderedFloat;
use std::hash::Hash;
use std::mem::MaybeUninit;
use std::{
    any::{Any, TypeId},
    sync::Mutex,
};

pub trait DefaultRef: Default + 'static {
    // needed for VarChar(s)
    const DEFAULT: Self;

    fn default_ref() -> &'static Self;
}

impl DefaultRef for Bool {
    const DEFAULT: Self = FALSE;

    fn default_ref() -> &'static Self {
        &Self::DEFAULT
    }
}

impl DefaultRef for Date {
    const DEFAULT: Self = date!(00010101);

    fn default_ref() -> &'static Self {
        &Self::DEFAULT
    }
}

impl DefaultRef for i32 {
    const DEFAULT: Self = 0;

    fn default_ref() -> &'static Self {
        &Self::DEFAULT
    }
}

impl DefaultRef for OrderedFloat<f64> {
    const DEFAULT: Self = OrderedFloat(0.0);

    fn default_ref() -> &'static Self {
        &Self::DEFAULT
    }
}

impl<const CAP: usize> DefaultRef for VarChar<CAP> {
    const DEFAULT: Self = Self::new_const();

    fn default_ref() -> &'static Self {
        &Self::DEFAULT
    }
}

static DEFAULTS: Lazy<Mutex<HashMap<TypeId, &'static (dyn Any + Sync)>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

impl<K, V, const ADD: Addition> DefaultRef for HashMap<K, V, ADD>
where
    K: Eq + Hash + Default + Sync + 'static,
    V: Default + Sync + 'static,
{
    // SAFETY: needed to satisfy trait - uninitialised bytes are never read
    #[allow(invalid_value)]
    const DEFAULT: Self = unsafe { MaybeUninit::<Self>::uninit().assume_init() };

    fn default_ref() -> &'static Self {
        let tid = TypeId::of::<Self>();
        let mut map = DEFAULTS.lock().unwrap();
        if let Some(existing) = map.get(&tid) {
            return (*existing as &dyn Any).downcast_ref::<Self>().unwrap();
        }
        let boxed: Box<Self> = Box::new(Self::default());
        let static_ref: &'static Self = Box::leak(boxed);
        map.insert(tid, static_ref as &(dyn Any + Sync));
        static_ref
    }
}

macro_rules! impl_default_ref_record {
    ($(($($ty:ident),+)),+ $(,)?) => {
        $(
            impl<$($ty),+> DefaultRef for Record<($($ty,)+)>
            where
                $($ty: Default + Sync + 'static),+
            {
                // SAFETY: needed to satisfy trait - uninitialised bytes are never read
                const DEFAULT: Self = unsafe { MaybeUninit::<Self>::uninit().assume_init() };

                fn default_ref() -> &'static Self {
                    let tid = TypeId::of::<Self>();
                    let mut map = DEFAULTS.lock().unwrap();
                    if let Some(existing) = map.get(&tid) {
                        return (*existing as &dyn Any).downcast_ref::<Self>().unwrap();
                    }
                    let boxed: Box<Self> = Box::new(Self::default());
                    let static_ref: &'static Self = Box::leak(boxed);
                    map.insert(tid, static_ref as &(dyn Any + Sync));
                    static_ref
                }
            }
        )+
    };
}

impl_default_ref_record! {
    (T1),
    (T1, T2),
    (T1, T2, T3),
    (T1, T2, T3, T4),
    (T1, T2, T3, T4, T5),
    (T1, T2, T3, T4, T5, T6),
    (T1, T2, T3, T4, T5, T6, T7),
    (T1, T2, T3, T4, T5, T6, T7, T8),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12),
}
