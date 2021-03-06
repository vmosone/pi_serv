use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use atom;
use mqtt;



fn call_3560614167(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in respond";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2256377725, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const mqtt::session::Session) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 913748025, true, param_error).expect("");
	let jst1 = *unsafe { Box::from_raw(ptr as *mut atom::Atom) };


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 104530634, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut Vec<u8>) };


    mqtt::session::Session::respond(jst0,jst1,jst2);
    Some(CallResult::Ok)
}

fn drop_1751456239(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut mqtt::server::ServerNode) };
}

fn drop_2256377725(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut mqtt::session::Session) };
}

fn drop_913748025(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut atom::Atom) };
}

fn drop_104530634(ptr: usize){
    unsafe { Box::from_raw(ptr as *mut Vec<u8>) };
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("mqtt::server::ServerNode"), drop_fn: drop_1751456239}, 1751456239);
    mgr.regist_struct_meta(StructMeta{name:String::from("mqtt::session::Session"), drop_fn: drop_2256377725}, 2256377725);
    mgr.regist_struct_meta(StructMeta{name:String::from("atom::Atom"), drop_fn: drop_913748025}, 913748025);
    mgr.regist_struct_meta(StructMeta{name:String::from("Vec<u8>"), drop_fn: drop_104530634}, 104530634);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3560614167), 3560614167);
}