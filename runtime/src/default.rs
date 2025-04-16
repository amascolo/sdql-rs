use once_cell::sync::Lazy;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Mutex,
};

pub trait DefaultRef: Default + 'static {
    fn default_ref() -> &'static Self;
}

static DEFAULTS: Lazy<Mutex<HashMap<TypeId, &'static (dyn Any + Sync)>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

impl<T> DefaultRef for T
where
    T: Default + Sync + 'static,
{
    fn default_ref() -> &'static Self {
        let type_id = TypeId::of::<T>();
        let mut map = DEFAULTS.lock().unwrap();

        if let Some(existing) = map.get(&type_id) {
            return (*existing as &dyn Any).downcast_ref().unwrap();
        }

        let boxed = Box::new(T::default());
        let static_ref = Box::leak(boxed);

        map.insert(type_id, static_ref as &(dyn Any + Sync));
        static_ref
    }
}

// TODO get rid of generic and variadic?
//  the above is simpler but might have a ~20% (?) performance impact (benchmark more carefully)
// impl DefaultRef for Bool {
//     fn default_ref() -> &'static Self {
//         &FALSE
//     }
// }
//
// impl DefaultRef for Date {
//     fn default_ref() -> &'static Self {
//         static DEFAULT: Date = date!(00010101);
//         &DEFAULT
//     }
// }
//
// impl DefaultRef for i32 {
//     fn default_ref() -> &'static Self {
//         &0
//     }
// }
//
// impl DefaultRef for OrderedFloat<f64> {
//     fn default_ref() -> &'static Self {
//         &OrderedFloat(0.0)
//     }
// }
//
// impl<const CAP: usize> DefaultRef for VarChar<CAP> {
//     fn default_ref() -> &'static Self {
//         let type_id = TypeId::of::<VarChar<CAP>>();
//         let mut map = DEFAULTS.lock().unwrap();
//
//         if let Some(existing) = map.get(&type_id) {
//             return (*existing as &dyn Any)
//                 .downcast_ref::<VarChar<CAP>>()
//                 .unwrap();
//         }
//
//         let boxed = Box::new(VarChar::<CAP>::default());
//         let static_ref: &'static VarChar<CAP> = Box::leak(boxed);
//
//         map.insert(type_id, static_ref as &(dyn Any + Sync));
//         static_ref
//     }
// }
//
// impl<T> DefaultRef for Record<(T,)>
// where
//     T: Default + Sync + 'static,
// {
//     fn default_ref() -> &'static Self {
//         let type_id = TypeId::of::<Record<(T,)>>();
//         let mut map = DEFAULTS.lock().unwrap();
//
//         if let Some(existing) = map.get(&type_id) {
//             return (*existing as &dyn Any)
//                 .downcast_ref::<Record<(T,)>>()
//                 .unwrap();
//         }
//
//         let boxed: Box<Record<(T,)>> = Box::new(Record::new((T::default(),)));
//         let static_ref: &'static Record<(T,)> = Box::leak(boxed);
//
//         map.insert(type_id, static_ref as &(dyn Any + Sync));
//         static_ref
//     }
// }
//
// macro_rules! impl_default_ref_record {
//     ($(($($ty:ident),+)),+ $(,)?) => {
//         $(
//             impl<$($ty),+> DefaultRef for Record<($($ty,)+)>
//             where
//                 $($ty: Default + Sync + 'static),+
//             {
//                 fn default_ref() -> &'static Self {
//                     let type_id = TypeId::of::<Record<($($ty,)+)>>();
//                     let mut map = DEFAULTS.lock().unwrap();
//
//                     if let Some(existing) = map.get(&type_id) {
//                         return (*existing as &dyn Any)
//                             .downcast_ref::<Record<($($ty,)+)>>()
//                             .unwrap();
//                     }
//
//                     let boxed: Box<Record<($($ty,)+)>> = Box::new(Record::new((
//                         $(
//                             <$ty as Default>::default(),
//                         )+
//                     )));
//                     let static_ref: &'static Record<($($ty,)+)> = Box::leak(boxed);
//
//                     map.insert(type_id, static_ref as &(dyn Any + Sync));
//                     static_ref
//                 }
//             }
//         )+
//     };
// }
//
// impl_default_ref_record! {
//     (T1),
//     (T1, T2),
//     (T1, T2, T3),
//     (T1, T2, T3, T4),
//     (T1, T2, T3, T4, T5),
//     (T1, T2, T3, T4, T5, T6),
//     (T1, T2, T3, T4, T5, T6, T7),
//     (T1, T2, T3, T4, T5, T6, T7, T8),
// }

// TODO get rid of these experiments
// /* ********************************** INDEX ********************************** */
// pub trait IndexDefaultGuard<'s, 'k, K> {
//     type Output;
//     fn get(this: &'s Self, key: &'k K) -> &'s Self::Output;
// }
// default impl<'s, 'k, K, V> IndexDefaultGuard<'s, 'k, K> for HashMap<K, V>
// where
//     K: Eq + Hash,
// {
//     type Output = V;
//
//     fn get(this: &'s Self, key: &'k K) -> &'s Self::Output {
//         // SAFETY: we know Self::Output = V
//         unsafe { &*(this.0.get(key).unwrap() as *const V as *const Self::Output) }
//     }
// }
// impl<'s, 'k, K, V> IndexDefaultGuard<'s, 'k, K> for HashMap<K, V>
// where
//     K: Eq + Hash,
//     V: DefaultRef + 'static,
// {
//     type Output = V;
//
//     fn get(this: &'s Self, key: &'k K) -> &'s Self::Output {
//         this.0.get(key).unwrap_or_else(|| V::default_ref())
//     }
// }
//
// impl<K, V> Index<&K> for HashMap<K, V>
// where
//     K: Eq + Hash,
//     for<'s, 'k> HashMap<K, V>: IndexDefaultGuard<'s, 'k, K, Output = V>,
// {
//     type Output = V;
//
//     fn index(&self, key: &K) -> &Self::Output {
//         <Self as IndexDefaultGuard<'_, '_, K>>::get(self, key)
//     }
// }
// /* ************************************************************************** */
