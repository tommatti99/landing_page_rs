use rocket;
use rocket::serde::json::Json;
use crate::collect_models::{CollectDataRequest, CollectDataResponse};

//=================================================================================
//  REQUEST:
//      Header: 
//          Content-Type: application/json
//      Body: 
//          {
//              name: String,
//              telephone_number: String,
//              email: String,
//              already_have_the_product: String,
//              want_to_receive_more_info: bool
//          }
//    
// RESPONSE:
//      Header: 
//          Content-Type: application/json
//      Body:
//          {
//              "success": bool,
//          }
//
#[post("/landing_page", format = "json", data = "<json_data>")]
pub fn collect_data_api(json_data: Json<CollectDataRequest>) -> Json<CollectDataResponse> {
    let data: CollectDataRequest = json_data.into_inner();
    
    return Json(CollectDataResponse::send_info(data));
}
//=================================================================================
