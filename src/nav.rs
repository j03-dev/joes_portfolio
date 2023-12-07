use leptos::*;
use stylers::style;

#[component]
pub fn Nav() -> impl IntoView {
    let style = style! {"Nav",
        nav {
            margin-top: 10px;
            width: 100%;
            display: flex;
            align-items: center;
        }
        nav ul {
            margin: 0;
            padding: 0;
            width: 100%;
            font-size: 20px;
            list-style: none;
            display: flex;
            justify-content: space-between;
            color: var(--primary-color);
        }
        .name {
            font-size: larger;
            font-weight: bold;
            color: var(--secondary-color);
        }

        header {
            width: 100%;
            height: 10%;
            display: flex;
            align-items: center;
        }
    };

    view! { class = style,
        <header>
            <nav>
                <ul>
                    <li class="name">FITAHIANA Nomeniavo Joe</li>
                    <li>Profile</li>
                    <li>Skill</li>
                    <li>Project</li>
                </ul>
            </nav>
        </header>
    }
}
