use futures_timer::Delay;
use std::collections::VecDeque;
use std::time::Duration;
use yew::platform::spawn_local;
use yew::{hook, use_callback, use_effect_with, use_state, Callback};

#[derive(Clone, PartialEq, Debug)]
pub struct ToastInfo {
    pub message: String,
    pub color: String,
}

#[derive(PartialEq, Clone)]
pub struct ToastControls {
    pub show_toast: Callback<ToastInfo>,
    pub current_toast: Option<ToastInfo>,
}

#[hook]
pub fn use_toast() -> ToastControls {
    let toast_info = use_state(VecDeque::<ToastInfo>::new);
    let current_toast = use_state(|| None);

    let show_toast = use_callback((toast_info.clone(),), |info: ToastInfo, (toast_info,)| {
        let mut new_list = (**toast_info).clone();
        new_list.push_back(info);
        toast_info.set(new_list);
    });

    use_effect_with(
        (toast_info, current_toast.clone()),
        |(toast_info, current_toast)| {
            let toast_info = toast_info.clone();
            let current_toast = current_toast.clone();
            spawn_local(async move {
                log::info!("1: {:?}", &toast_info);
                if toast_info.is_empty() {
                    return;
                }
                let mut new_list = (*toast_info).clone();
                let popped_toast = new_list.pop_front();
                toast_info.set(new_list);
                current_toast.set(popped_toast);
                log::info!("{:?}", &toast_info);
                Delay::new(Duration::from_secs(5)).await;
                current_toast.set(None);
            })
        },
    );

    ToastControls {
        show_toast,
        current_toast: (*current_toast).clone(),
    }
}
