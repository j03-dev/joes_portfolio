use leptos::*;
use stylers::style;

#[component]
fn Left() -> impl IntoView {
    let style = style! {"Left",
        .left {
            width: 50%;
            height: 100%;
            margin: auto;
            display: grid;
            place-content: center;
        }

        .left-top {
            display: grid;
            color: var(--secondary-color);
            padding-bottom: 20px;
        }

        .left-bottom {
            width: 50%;
            display: flex;
            gap: "1em"
        }

        h1 {
            font-size: 70px;
            padding-bottom: 20px;
        }

        button {
            width: 300px;
            height: 40px;
            border: none;
            border-radius: 40px;
        }

        .left-bottom button:first-child {
            color: var(--bg-color);
            background-color: var(--primary-color);
        }

        .left-bottom button:last-child {
            color: var(--bg-color);
            background-color: var(--secondary-color);
        }

    };
    view! { class = style,
        <div class="left">
            <div class="left-top">
                <h1>Fullstack Developer</h1>
                <p>Hello my name is <b>FITAHIANA Nomeniavo Joe</b>, but you can call me <b>Joe</b>, Im Fullstack developer, </p>
            </div>

            <div class="left-bottom">
                <button>Me contacter</button>
                <button>Télécharger le CV</button>
            </div>
        </div>
    }
}

#[component]
fn Right() -> impl IntoView {
    let style = style! {"Right",
        .right {
            width: 50%;
            display: grid;
            place-content: center;
        }

        img {
            width: 50vh;
            height: 50vh;
            object-fit: cover;
            border-radius: 300px;
        }
    };
    view! { class = style,
        <div class="right">
            <div class="image-contanier">
                <img src="img/image.jpg" alt="image-profile"></img>
            </div>
        </div>
    }
}

#[component]
pub fn Profile() -> impl IntoView {
    let style = style! {"Profile",
        section{
            display: flex;
            height: 100%;
        }
    };
    view! { class = style,
        <section>
            <Left/>
            <Right/>
        </section>
    }
}
