pub fn generate() -> proc_macro2::TokenStream {
    quote::quote! {
        fn msg_cycles_accept(
            _this: &boa_engine::JsValue,
            aargs: &[boa_engine::JsValue],
            context: &mut boa_engine::Context,
        ) -> boa_engine::JsResult<boa_engine::JsValue> {
            let max_amount: u64 = aargs
                .get(0)
                .ok_or_else(|| "An argument for 'maxAmount' was not provided".to_js_error(None))?
                .clone()
                .try_from_vm_value(&mut *context)?;

            let return_value: boa_engine::bigint::JsBigInt =
                ic_cdk::api::call::msg_cycles_accept(max_amount).into();

            Ok(return_value.into())
        }
    }
}
