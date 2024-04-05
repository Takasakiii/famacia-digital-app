use crate::components::button::Button;
use crate::components::input::Input;
use crate::components::logo::Logo;
use crate::components::screen_center::ScreenCenter;
use crate::components::screen_padding::ScreenPadding;
use crate::components::spacer::Spacer;
use crate::hooks::toast_hook::{ToastControls, ToastInfo};
use crate::routes::Routes;
use crate::utils::call_tauri;
use serde::Serialize;
use yew::platform::spawn_local;
use yew::{function_component, html, use_context, use_state, Callback, Html};
use yew_router::hooks::use_navigator;

#[derive(Serialize)]
struct LoginTauri<'a> {
    email: &'a str,
    password: &'a str,
}

#[function_component(LoginScreen)]
pub fn login_screen() -> Html {
    let email = use_state(String::new);
    let password = use_state(String::new);
    let navigator = use_navigator().unwrap();
    let toast = use_context::<ToastControls>().expect("no toast context");

    let void_callback = Callback::from(move |_| {});

    let login_callback = {
        let email = (*email).clone();
        let password = (*password).clone();
        Callback::from(move |_| {
            let email = email.clone();
            let password = password.clone();
            let navigator = navigator.clone();
            let toast = toast.clone();
            spawn_local(async move {
                let is_logged: bool = call_tauri(
                    "login",
                    &LoginTauri {
                        email: &email,
                        password: &password,
                    },
                )
                .await;

                if is_logged {
                    navigator.push(&Routes::Pharmacies);
                } else {
                    toast.show_toast.emit(ToastInfo {
                        message: "Falha ao logar".to_string(),
                        color: "is-danger".to_string(),
                    })
                }
            });
        })
    };

    html! {
        <ScreenCenter>
            <ScreenPadding>
                // <Title title="Farmacia Digital" center={true} />
                <Logo center={true} is_title={true} />
                <Spacer height="1.5rem" />
                <Input label="Email" input_type="email" placeholder="test@gmail.com" value_state={email} />
                <Input label="Senha" input_type="password" placeholder="•••••••••" value_state={password} />
                <Spacer height="1.5rem" />
                <Button text="Entrar" color="is-primary" full_width={true} on_click={login_callback} />
                <Spacer height="1rem" />
                <Button text="Cadastrar-se" color="is-link" full_width={true} on_click={void_callback.clone()} />
                <Spacer height="1rem" />
                <Button text="Esqueci minha senha" full_width={true} on_click={void_callback.clone()}  />
            </ScreenPadding>
        </ScreenCenter>
    }
}
