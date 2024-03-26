use crate::components::button::Button;
use crate::components::input::Input;
use crate::components::screen_center::ScreenCenter;
use crate::components::screen_padding::ScreenPadding;
use crate::components::spacer::Spacer;
use crate::components::title::Title;
use yew::{function_component, html, Html};

#[function_component(LoginScreen)]
pub fn login_screen() -> Html {
    html! {
        <ScreenCenter>
            <ScreenPadding>
                <Title title="Farmacia Digital" center={true} />
                <Spacer height="1.5rem" />
                <Input label="Email" input_type="email" placeholder="test@gmail.com" />
                <Input label="Senha" input_type="password" placeholder="•••••••••" />
                <Spacer height="1.5rem" />
                <Button text="Entrar" color="is-primary" full_width={true} />
                <Spacer height="1rem" />
                <Button text="Cadastrar-se" color="is-link" full_width={true} />
                <Spacer height="1rem" />
                <Button text="Esqueci minha senha" full_width={true} />
            </ScreenPadding>
        </ScreenCenter>
    }
}
