use crate::nav::Nav;
use crate::profile::Profile;
use leptos::*;
use stylers::style;

#[component]
pub fn App() -> impl IntoView {
    let style = style! {"App",
        :root {
            --bg-color: #d1cfcc;
            --primary-color: #c77430;
            --secondary-color: #314c4a;
        }

        .container {
            width: 65%;
            height: 100vh;
            margin: auto;
        }
    };
    view! { class = style,
        <div class="container">
            <Nav/>
            <Profile/>
        </div>
    }
}
