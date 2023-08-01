pub fn generate() -> proc_macro2::TokenStream {
    quote::quote! {
        fn clear_timer(
            _this: &boa_engine::JsValue,
            aargs: &[boa_engine::JsValue],
            context: &mut boa_engine::Context,
        ) -> boa_engine::JsResult<boa_engine::JsValue> {
            let timer_id: ic_cdk_timers::TimerId = aargs
                .get(0)
                .ok_or_else(|| "An argument for 'id' was not provided".to_js_error(None))?
                .clone()
                .try_from_vm_value(&mut *context)?;

            ic_cdk_timers::clear_timer(timer_id);

            Ok(boa_engine::JsValue::Undefined)
        }
    }
}
