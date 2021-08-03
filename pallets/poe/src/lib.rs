//编译标签
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

///存证模块
#[frame_support::pallet]
pub mod pallet {

    // 引入依赖
    // frame_support::dispatch::DispatchResultWithPostInfo 可调用函数的返回结果
    // frame_support::pallet_prelude::* 常用宏
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    // 系统模块，数据和类型信息
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;
    // use frame_support::sp_runtime::traits::StaticLookup;
    use sp_runtime::{
        traits::{
            StaticLookup
        }
    };

    // 定义模块配置接口，继承自系统模块的Config接口
    #[pallet::config]
    pub trait Config: frame_system::Config {

        // /// Because this pallet emits events, it depends on the runtime's definition of an event.
        // 关联类型
        // 可以从模块的Event类型进行转换，并 是系统模块的Event类型
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    // 定义Pallet结构体承载功能模块
    // 依赖存储单元，
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // 定义存储单元，用来存储存正
    // 定义可选择 get函数 proofs
    //  _ prefix
    //  Blake2_128Concat key哈希
    // Vec<u8> key类型
    // (T::AccountId, T::BlockNumber), 帐号，区块 来自系统模块
    // ValueQuery
    #[pallet::storage]
    #[pallet::getter(fn proofs)]
    pub(super) type Proofs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Vec<u8>,
        (T::AccountId, T::BlockNumber),
        // ValueQuery
    >;

    // 定义事件枚举
    // 宏pallet::generate_deposit 生成帮助性的方法，deposit_event方便的进行Event的触发
    // In FRAME v2.
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // /// Set a value.
        // ValueSet(u32, T::AccountId),

        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRevoked(T::AccountId, Vec<u8>),
        ClaimTransfered(T::AccountId, T::AccountId, Vec<u8>),
    }

    // 定义Error枚举
    #[pallet::error]
    pub enum Error<T> {
        // /// Error names should be descriptive.
        // NoneValue,
        // /// Errors should have helpful documentation associated with them.
        // StorageOverflow,
        /// 已存在
        ProofAlreadyExist,
        ClaimNotExist,
        NotClaimOwner,
    }

    // // FRAME v2.
    // #[pallet::error]
    // pub enum Error<T> {
    //     /// Error names should be descriptive.
    //     InvalidParameter,
    //     /// Errors should have helpful documentation associated with them.
    //     OutOfSpace,
    // }

    // 定义hooks空实现
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // 定义可调用函数
    #[pallet::call]
    impl<T:Config> Pallet<T> {

        // 创建存证
        #[pallet::weight(0)]
        pub fn create_claim(
            origin: OriginFor<T>,
            claim: Vec<u8>
        ) -> DispatchResultWithPostInfo {

            // 校验发送方
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let sender = ensure_signed(origin)?;

            ensure!(!Proofs::<T>::contains_key(&claim),Error::<T>::ProofAlreadyExist);

            Proofs::<T>::insert(
                &claim,
                (sender.clone(), frame_system::Pallet::<T>::block_number()),
            );

            // // Update storage.
            // <Something<T>>::put(something);

            // Emit an event.
            Self::deposit_event(Event::ClaimCreated(sender, claim));
            // Return a successful DispatchResultWithPostInfo
            Ok(().into())
        }

        /// 吊销存证
        #[pallet::weight(0)]
        pub fn revoke_claim(
            origin: OriginFor<T>,
            claim: Vec<u8>
        ) -> DispatchResultWithPostInfo {

            // 校验发送方
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let sender = ensure_signed(origin)?;

            let (owner,_)=Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
            ensure!(sender == owner,Error::<T>::NotClaimOwner);

            // // Update storage.
            // 删除存证
            Proofs::<T>::remove(&claim);


            // Emit an event.
            Self::deposit_event(Event::ClaimRevoked(sender, claim));
            // Return a successful DispatchResultWithPostInfo
            Ok(().into())
        }

        // 转移存证
        #[pallet::weight(0)]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            claim: Vec<u8>,
            dest: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResultWithPostInfo {

            // 校验发送方
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let sender = ensure_signed(origin)?;
            let dest = T::Lookup::lookup(dest)?;
            let (owner,_)=Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
            ensure!(sender == owner,Error::<T>::NotClaimOwner);

            Proofs::<T>::remove(&claim);

            // let dest = T::Lookup::lookup(dest)?;
            Proofs::<T>::insert(
                &claim,
                (dest.clone(), frame_system::Pallet::<T>::block_number()),
            );

            // // Update storage.
            // <Something<T>>::put(something);

            // Emit an event.
            Self::deposit_event(Event::ClaimTransfered(sender, dest, claim));
            // Return a successful DispatchResultWithPostInfo
            Ok(().into())
        }
    }

}