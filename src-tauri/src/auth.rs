use gamercade_interface::{auth::{auth_service_client::AuthServiceClient, login_request::Provider, LoginRequest}, Session};

use super::SERVICE_IP_GRPC;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
  };

#[tauri::command]
async fn try_login(username: String, password: String) {
    println!("trying to login...");
    let mut client = AuthServiceClient::connect(SERVICE_IP_GRPC).await.unwrap();

    let request = LoginRequest {
        provider: Some(Provider::Username(username)),
        password,
    };

    let response = client.login(request).await;

    match response {
        Ok(ok) => {
            let response = ok.into_inner();
            let session = Session::from(response.session);
            println!("logged in successfully!");
        },
        Err(err) => println!("try_login error: {err}"),
    }
}

pub fn auth_plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("auth")
      .invoke_handler(tauri::generate_handler![try_login])
      .build()
  }