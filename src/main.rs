use core::fmt::Display;
use std::time::Duration;
use std::{collections::HashMap, hash::Hash};
use leptos::ev::{SubmitEvent};
use leptos::html::{Input, Canvas};

use enum_iterator::{all, Sequence};
use leptos::*;
use leptos_router::*;
use log::info;

use wasm_bindgen::prelude::*;
use web_sys::Event;
use std::f64;

fn main() {
    femme::start();
    mount_to_body(|cx| {
        view! {
        cx, <App/>}
    })
}
#[component]
fn App(cx: Scope) -> impl IntoView {
   // setup_background(cx);
    view! {cx,
        <Router>
            <nav>
                <Navbar/>
            </nav>
            <main>
                <Routes>
                    <Route path="" view=move |_| view!{cx, 
                        <Page id="landing_page">
                            <Landing/>
                        </Page>
                    } />
                    <Route path="projects" view=move |_| view!{cx, 
                        <Page id="projects">
                            <Projects/>
                        </Page>
                    } />
                    <Route path="about" view=move |_| view!{cx, 
                        <Page id="about">
                            <About/>
                        </Page>
                    } />
                    <Route path="gallery" view=move |_| view!{cx, 
                        <Page id="gallery">
                            <Gallery/>
                        </Page>
                    } />
                    <Route path="team" view=move |_| view!{cx, 
                        <Page id="team">
                            <Team/>
                        </Page>
                    } />
                    <Route path="register" view=move |_| view!{cx, 
                        <Page id="register">
                            <Register/>
                        </Page>
                    } />
                </Routes>
            </main>
        </Router>
    }
}

fn setup_background(cx: Scope) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("background").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context: web_sys::CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let (context, set_context) = create_signal(cx, context);
    window_event_listener(ev::resize, move |ev| {
        set_context(canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap());
    });

    create_effect(cx, move |a: Option<i32>| {
    context.with(|ctx| {
    let context = ctx;
    context.begin_path();
    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0,f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
            if let Some(a) = a {
                return a + 1;
            } else {
                0
            }
    })
    });
}

#[component]
fn Register(cx: Scope) -> impl IntoView {
    let first_name: NodeRef<Input> = create_node_ref(cx);
    let last_name: NodeRef<Input> = create_node_ref(cx);
    let mob_number: NodeRef<Input> = create_node_ref(cx);
    let email_address: NodeRef<Input> = create_node_ref(cx);
    let roll_no: NodeRef<Input> = create_node_ref(cx);
    let register = move |ev: SubmitEvent| {
        ev.prevent_default();
    };
    view!{cx, 
        <img src="/images/logo.svg" /> 
        <div class="heading">
            Sign In
        </div>
        <form on:submit=register>
            <input type="text" node_ref=first_name placeholder="First Name" />
            <input type="text" node_ref=last_name placeholder="Last Name" />
            <input type="" node_ref=roll_no placeholder="Roll No." />
            <input type="text" node_ref=mob_number placeholder="Phone Number"/>
            <input type="email" node_ref=email_address placeholder="E-Mail" />
            <button type="submit">Submit</button>
        </form>
    }
}

#[derive(PartialEq, Clone, Copy, Eq, Sequence, Hash, Debug)]
enum TeamSections {
    Faculty,
    Core,
    Alumni,
    Members,
}

impl Display for TeamSections {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Faculty => f.write_str("Faculty"),
            Self::Core => f.write_str("Core"),
            Self::Members => f.write_str("Members"),
            Self::Alumni => f.write_str("Alumni"),
        }
    }
}

struct TeamMember {
    name: String,
    rank: Option<Rank>,
    deptt: Deptt,
    roll_no: Option<String>,
    mail: Option<String>,
    phone: Option<String>,
    photo: String,
    socials: Option<String>,
}

struct TeamMemberState {
    name: RwSignal<String>,
    rank: RwSignal<Option<Rank>>,
    deptt: RwSignal<Deptt>,
    roll_no: RwSignal<Option<String>>,
    mail: RwSignal<Option<String>>,
    phone: RwSignal<Option<String>>,
    photo: RwSignal<String>,
    socials: RwSignal<Option<String>>,
    is_hidden: RwSignal<bool>,
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
enum Deptt {
    GEE,
    GEC,
    GME,
    GCS,
    GFT,
    GCT,
    GIN,
    CDE
}

impl Display for Deptt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::GCT => f.write_str("GCT"),
            Self::GCS => f.write_str("GCS"),
            Self::GFT => f.write_str("GFT"),
            Self::GEE => f.write_str("GEE"),
            Self::GEC => f.write_str("GEC"),
            Self::GME => f.write_str("GME"),
            Self::GIN => f.write_str("GIN"),
            Self::CDE => f.write_str("DCS - CDE"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
enum Rank {
    FacultyAdvisor,
    SenCoordinator,
    SocialMedia,
    MechanicalHead,
    WebDev,
}

impl ToString for Rank {
    fn to_string(&self) -> String {
        match self {
            Self::FacultyAdvisor => "Faculty Advisor",
            Self::SenCoordinator => "Senior Coordinator",
            Self::SocialMedia => "Social Media",
            Self::MechanicalHead => "Mechanical Head",
            Self::WebDev => "Web Development",
        }
        .to_string()
    }
}

#[component]
fn Team(cx: Scope) -> impl IntoView {
    let content = HashMap::from([
        (
            TeamSections::Faculty,
            vec![TeamMember {
                name: "Dr. J.S. Ubhi".to_string(),
                rank: Some(Rank::FacultyAdvisor),
                deptt: Deptt::GEC,
                photo: "/images/JS UBHI.jpg".to_string(),
                phone: None,
                mail: None,
                roll_no: None,
                socials: None,
            }],
        ),
        (
            TeamSections::Alumni,
            vec![
                TeamMember {
                    name: "Aman".to_string(),
                    rank: None,
                    deptt: Deptt::GME,
                    photo: "/images/AMAN.png".to_string(),
                    phone: None,
                    mail: Some("2140324@sliet.ac.in".to_string()),
                    roll_no: Some("1940303".to_string()),
                    socials: Some("https://www.linkedin.com/in/aman-421335192".to_string())
                },
                TeamMember {
                    name: "Saurabh Kumar".to_string(),
                    rank: None,
                    deptt: Deptt::GME,
                    photo: "/images/SAURABH KUMAR.jpg".to_string(),
                    phone: None,
                    mail: Some("2040376@sliet.ac.in".to_string()),
                    roll_no: Some("2040376".to_string()),
                    socials: Some("https://www.linkedin.com/in/saurabh-kumar-7004b4239".to_string())
                },
                TeamMember {
                    name: "Ajay Kumar".to_string(),
                    rank: None,
                    deptt: Deptt::GME,
                    photo: "/images/AJAY KUMAR.jpg".to_string(),
                    phone: None,
                    mail: Some("1830290@sliet.ac.in".to_string()),
                    roll_no: Some("1830290".to_string()),
                    socials: Some("https://www.linkedin.com/in/ajay-kumar-944892193".to_string())
                },
                TeamMember {
                    name: "Dhanesh Kumar".to_string(),
                    rank: None,
                    deptt: Deptt::GEC,
                    photo: "/images/DHANESH KUMAR.jpg".to_string(),
                    phone: None,
                    mail: Some("1840061@sliet.ac.in".to_string()),
                    roll_no: Some("1840061".to_string()),
                    socials: Some("https://www.linkedin.com/in/dhanesh-verma-82662a16b".to_string())
                },
                TeamMember {
                    name: "Akshat Garg".to_string(),
                    rank: None,
                    deptt: Deptt::GCS,
                    photo: "/images/Akshat Garg.jpg".to_string(),
                    phone: None,
                    mail: None,
                    roll_no: Some("1740571".to_string()),
                    socials: Some("linkedin.com/in/iAkshatGarg".to_string())
                },
                TeamMember {
                    name: "Dhruv Patel".to_string(),
                    rank: None,
                    deptt: Deptt::GEE,
                    photo: "/images/Dhruv Patel.jpg".to_string(),
                    phone: None,
                    mail: None,
                    roll_no: Some("GEE/SL/15/4309".to_string()),
                    socials: None
                },
                TeamMember {
                    name: "Kaushik Suman".to_string(),
                    rank: None,
                    deptt: Deptt::GEE,
                    photo: "/images/Kaushik Suman.JPG".to_string(),
                    phone: None,
                    mail: Some("kaushiksuman001@gmail.com".to_string()),
                    roll_no: Some("GEE/SL/15/1403".to_string()),
                    socials: Some("linkedin.com/in/kaushik-suman-596427164".to_string())
                },
                TeamMember {
                    name: "Muskan Methi".to_string(),
                    rank: None,
                    deptt: Deptt::GEE,
                    photo: "/images/Muskan Methi.jpg".to_string(),
                    phone: None,
                    mail: None,
                    roll_no: Some("1740708".to_string()),
                    socials: Some("https://www.linkedin.com/in/muskan-methi-519738193".to_string())
                },
                TeamMember {
                    name: "Apoorv Bansal".to_string(),
                    rank: None,
                    deptt: Deptt::GME,
                    photo: "/images/APOORV BANSAL.jpg".to_string(),
                    phone: None,
                    mail: Some("1940320@sliet.ac.in".to_string()),
                    roll_no: Some("1940320".to_string()),
                    socials: Some("https://www.linkedin.com/in/apoorv-bansal-3876421a6".to_string())
                },
            ],
        ),
        (
            TeamSections::Core,
            vec![
                TeamMember {
                    name: "Manish Kumar".to_string(),
                    rank: Some(Rank::SenCoordinator),
                    deptt: Deptt::GME,
                    photo: "/images/MANISH KUMAR.jpg".to_string(),
                    phone: Some("8507736011".to_string()),
                    mail: None,
                    roll_no: Some("2040301".to_string()),
                    socials: Some("https://www.linkedin.com/in/manish-krishnaut".to_string()),
                },
                TeamMember {
                    name: "Pratham Himanshu".to_string(),
                    rank: Some(Rank::SenCoordinator),
                    deptt: Deptt::GIN,
                    photo: "/images/PRATHAM HIMANSHU.jpg".to_string(),
                    phone: Some("9060312892".to_string()),
                    mail: Some("2140259@sliet.ac.in".to_string()),
                    roll_no: Some("2040301".to_string()),
                    socials: Some("https://www.linkedin.com/in/pratham-himanshu-394312258".to_string()),
                },
                TeamMember {
                    name: "Ritesh Prasad".to_string(),
                    rank: Some(Rank::SenCoordinator),
                    deptt: Deptt::GEE,
                    photo: "/images/RITESH PRASAD.jpg".to_string(),
                    phone: Some("6201968063".to_string()),
                    mail: Some("2140386@sliet.ac.in".to_string()),
                    roll_no: Some("2140386".to_string()),
                    socials: Some("https://www.linkedin.com/in/ritesh-prasad-8956a9228".to_string()),
                },
                TeamMember {
                    name: "Ram Kumar".to_string(),
                    rank: Some(Rank::SenCoordinator),
                    deptt: Deptt::GME,
                    photo: "/images/RAM KUMAR.jpg".to_string(),
                    phone: Some("6203884073".to_string()),
                    mail: Some("2040367@sliet.ac.in".to_string()),
                    roll_no: Some("2040367".to_string()),
                    socials: Some("https://www.linkedin.com/in/ram-kumar-06345a202".to_string()),
                },
                TeamMember {
                    name: "Sushant Shankar".to_string(),
                    rank: Some(Rank::SenCoordinator),
                    deptt: Deptt::GME,
                    photo: "/images/SUSHANT SHANKAR.jpg".to_string(),
                    phone: Some("7646045777".to_string()),
                    mail: Some("2140317@sliet.ac.in".to_string()),
                    roll_no: Some("2140317".to_string()),
                    socials: Some("https://www.linkedin.com/in/sushant-shankar-a49007224/".to_string()),
                },
            ],
        ),
        (
            TeamSections::Members,
            vec![
                TeamMember {
                    name: "Priyanshu Kumar Jha".to_string(),
                    rank: None,
                    deptt: Deptt::GEE,
                    photo: "/images/PRIYANSHU KUMAR JHA.jpg".to_string(),
                    phone: Some("6299326241 ".to_string()),
                    mail: Some("2140313@sliet.ac.in".to_string()),
                    roll_no: Some("2140313".to_string()),
                    socials: Some("https://linkedin.com/in/priyanshu-kumar-jha-997808229".to_string()),
                },
                TeamMember {
                    name: "Shreya Sinha".to_string(),
                    rank: None,
                    deptt: Deptt::GEC,
                    photo: "/images/SHREYA SINHA.jpg".to_string(),
                    phone: None,
                    mail: Some("2140255@sliet.ac.in".to_string()),
                    roll_no: Some("2140255".to_string()),
                    socials: Some("https://www.linkedin.com/in/shreya-sinha-442a8b248".to_string()),
                },
                TeamMember {
                    name: "Amisha Nath".to_string(),
                    rank: None,
                    deptt: Deptt::GCS,
                    photo: "/images/AMISHA NATH.jpg".to_string(),
                    phone: None,
                    mail: None,
                    roll_no: Some("2140116".to_string()),
                    socials: Some("https://www.linkedin.com/in/ankit-sharma-97888a224".to_string()),
                },
                TeamMember {
                    name: "Prins Singh".to_string(),
                    rank: None,
                    deptt: Deptt::GEC,
                    photo: "/images/PRINS SINGH.jpeg".to_string(),
                    phone: Some("9335738588".to_string()),
                    mail: Some("2243009@sliet.ac.in".to_string()),
                    roll_no: Some("2243009".to_string()),
                    socials: Some("https://www.linkedin.com/in/prins-singh-a6ab59229".to_string()),
                },
                TeamMember {
                    name: "Prabhdeep Singh".to_string(),
                    rank: None,
                    deptt: Deptt::GEC,
                    photo: "/images/PRABHDEEP SINGH.jpg".to_string(),
                    phone: Some("9855812076".to_string()),
                    mail: Some("prabhdeep5singh555@sliet.ac.in".to_string()),
                    roll_no: Some("2243015".to_string()),
                    socials: None,
                },
                TeamMember {
                    name: "Utkarsh Kumar Mishra".to_string(),
                    rank: None,
                    deptt: Deptt::GEC,
                    photo: "/images/UTKARSH KUMAR MISHRA.jpg".to_string(),
                    phone: Some("9315533675".to_string()),
                    mail: Some("2236024@sliet.ac.in".to_string()),
                    roll_no: Some("2236024".to_string()),
                    socials: Some("https://www.linkedin.com/in/utkarsh-kumar-mishra-27b279222".to_string()),
                },
                TeamMember {
                    name: "Ashish Kumar".to_string(),
                    rank: None,
                    deptt: Deptt::GEE,
                    photo: "/images/ASHISH KUMAR.jpg".to_string(),
                    phone: Some("9828917506".to_string()),
                    mail: Some("2040308@sliet.ac.in".to_string()),
                    roll_no: Some("2040308".to_string()),
                    socials: Some("www.linkedin.com/in/ ashish-kumar-b99487251".to_string()),
                },
                TeamMember {
                    name: "Vikas Chaurasia".to_string(),
                    rank: None,
                    deptt: Deptt::GCS,
                    photo: "/images/VIKAS CHAURASIA.jpg".to_string(),
                    phone: Some("8858444912".to_string()),
                    mail: None,
                    roll_no: Some("2140130".to_string()),
                    socials: Some("linkedin.com/in/vikascc28".to_string()),
                },
                TeamMember {
                    name: "Dheeraj Kumar".to_string(),
                    rank: None,
                    deptt: Deptt::GME,
                    photo: "/images/DHEERAJ KUMAR.jpg".to_string(),
                    phone: Some("8858444912".to_string()),
                    mail: None,
                    roll_no: Some("2140130".to_string()),
                    socials: Some("linkedin.com/in/vikascc28".to_string()),
                },
                TeamMember {
                    name: "Sushil Kumar Yadav".to_string(),
                    rank: None,
                    deptt: Deptt::GME,
                    photo: "/images/SHUSHIL KUMAR YADAV.jpg".to_string(),
                    phone: Some("7079155670 ".to_string()),
                    mail: Some("2140371@sliet.ac.in".to_string()),
                    roll_no: Some("2140371".to_string()),
                    socials: Some("Linkedin.com/in/shushil kumar".to_string()),
                },
                TeamMember {
                    name: "Souradip Das".to_string(),
                    rank: None,
                    deptt: Deptt::GEC,
                    photo: "/images/Souradip Das.jpg".to_string(),
                    phone: Some("8640905340 ".to_string()),
                    mail: None,
                    roll_no: Some("2243032".to_string()),
                    socials: Some("https://www.linkedin.com/in/souradip-das-077a72256".to_string()),
                },
                TeamMember {
                    name: "Aman Kumar".to_string(),
                    rank: None,
                    deptt: Deptt::GEE,
                    photo: "/images/AMAN.png".to_string(),
                    phone: Some("9304061281".to_string()),
                    mail: Some("2140324@sliet.ac.in".to_string()),
                    roll_no: Some("2140324".to_string()),
                    socials: Some("https://www.linkedin.com/in/aman-kumar-087463243".to_string()),
                },
                TeamMember {
                    name: "Shubham Kumar".to_string(),
                    rank: None,
                    deptt: Deptt::GIN,
                    photo: "/images/SHUBHAM KUMAR.png".to_string(),
                    phone: Some("6205538058".to_string()),
                    mail: Some("2244212@sliet.ac.in".to_string()),
                    roll_no: Some("2244212".to_string()),
                    socials: Some("https://www.linkedin.com/in/shubham-kumar-srivastav-b46831224/".to_string()),
                },
                TeamMember {
                    name: "Ankit Sharma".to_string(),
                    rank: None,
                    deptt: Deptt::GIN,
                    photo: "/images/ANKIT SHARMA.jpg".to_string(),
                    phone: Some("6377323373 ".to_string()),
                    mail: None,
                    roll_no: Some("2140116".to_string()),
                    socials: Some("https://www.linkedin.com/in/ankit-sharma-97888a224".to_string()),
                },
                TeamMember {
                    name: "Adarsh Kumar Roy".to_string(),
                    rank: None,
                    deptt: Deptt::GME,
                    photo: "/images/ADARSH KUMAR ROY.jpg".to_string(),
                    phone: Some("8902484925".to_string()),
                    mail: Some("2140389@sliet.ac.in".to_string()),
                    roll_no: Some("2140389".to_string()),
                    socials: Some("https://www.linkedin.com/in/adarsh-kumar-roy-99851a1a5".to_string()),
                },
                TeamMember {
                    name: "Puja Kumari".to_string(),
                    rank: None,
                    deptt: Deptt::GFT,
                    photo: "/images/PUJA KUMARI.jpg".to_string(),
                    phone: None,
                    mail: Some("2245015@sliet.ac.in".to_string()),
                    roll_no: Some("2245015".to_string()),
                    socials: Some("LinkedIn.com/in/pujakumari".to_string()),
                },
                TeamMember {
                    name: "Arbind Kumar".to_string(),
                    rank: None,
                    deptt: Deptt::GME,
                    photo: "/images/Arbind Kumar.jpg".to_string(),
                    phone: Some("8303731599".to_string()),
                    mail: Some("2110204@sliet.ac.in".to_string()),
                    roll_no: Some("2140397".to_string()),
                    socials: Some("https://www.linkedin.com/in/arbind-kumar-18569425b".to_string()),
                },
                TeamMember {
                    name: "Anisha Kumari".to_string(),
                    rank: None,
                    deptt: Deptt::CDE,
                    photo: "/images/ANISHA KUMARI.jpg".to_string(),
                    phone: None,
                    mail: Some("2110204@sliet.ac.in".to_string()),
                    roll_no: Some("2110204".to_string()),
                    socials: None,
                },
                TeamMember {
                    name: "Amisha".to_string(),
                    rank: None,
                    deptt: Deptt::GEE,
                    photo: "/images/AMISHA.jpg".to_string(),
                    phone: None,
                    mail: Some("2140311@sliet.ac.in".to_string()),
                    roll_no: Some("2140311".to_string()),
                    socials: Some("https://www.linkedin.com/in/amisha-96627a243".to_string()),
                },
                TeamMember {
                    name: "Pahul Raj Singh".to_string(),
                    rank: None,
                    deptt: Deptt::GEE,
                    photo: "/images/PAHULRAJ SINGH.jpeg".to_string(),
                    phone: Some("8750351447".to_string()),
                    mail: Some("2140306@sliet.ac.in".to_string()),
                    roll_no: Some("2140306".to_string()),
                    socials: Some("https://www.linkedin.com/in/pahul-raj-singh-614970228?lipi=urn%3Ali%3Apage%3Ad_flagship3_profile_view_base_contact_details%3BJMRt8b7HSKidu55iihjzbg%3D%3D".to_string()),
                },
                TeamMember {
                    name: "Shreyansh Abhishek".to_string(),
                    rank: None,
                    deptt: Deptt::GME,
                    photo: "/images/SHREYANSH ABHISHEK.jpg".to_string(),
                    phone: Some("7992433975".to_string()),
                    mail: Some("2140359@sliet.ac.in".to_string()),
                    roll_no: Some("2140359".to_string()),
                    socials: Some("www.linkedin.com/in/shreyansh-abhishek-878977240".to_string()),
                },
                TeamMember {
                    name: "Aadarsh Nath".to_string(),
                    rank: None,
                    deptt: Deptt::CDE,
                    photo: "/images/Aadarsh Nath.JPG".to_string(),
                    phone: Some("8810510857".to_string()),
                    mail: None,
                    roll_no: Some("2110238".to_string()),
                    socials: Some("www.linkedin.com/in/aadarsh-nath-7007".to_string()),
                },
                TeamMember {
                    name: "Mehak Negi".to_string(),
                    rank: None,
                    deptt: Deptt::CDE,
                    photo: "/images/MEHAK NEGI.JPG".to_string(),
                    phone: None,
                    mail: Some("2211021@sliet.ac.in".to_string()),
                    roll_no: Some("2211021".to_string()),
                    socials: Some("https://www.linkedin.com/in/mehak-negi-935985261".to_string()),
                },
                TeamMember {
                    name: "Shruti Kumari".to_string(),
                    rank: None,
                    deptt: Deptt::CDE,
                    photo: "/images/SHRUTI KUMARI.jpg".to_string(),
                    phone: None,
                    mail: Some("2211075@sliet.ac.in".to_string()),
                    roll_no: Some("2211075".to_string()),
                    socials: Some("https://www.linkedin.com/in/shruti-jaiswal-bb3146263".to_string()),
                },
                TeamMember {
                    name: "Rishav Raj".to_string(),
                    rank: None,
                    deptt: Deptt::CDE,
                    photo: "/images/RISHAV RAJ.png".to_string(),
                    phone: Some("7091878889".to_string()),
                    mail: Some("2213103@sliet.ac.in".to_string()),
                    roll_no: Some("2213103".to_string()),
                    socials: Some("https://www.linkedin.com/in/-mr-rishav".to_string()),
                },
                TeamMember {
                    name: "Sanit Arya".to_string(),
                    rank: None,
                    deptt: Deptt::CDE,
                    photo: "/images/SANIT ARYA.jpg".to_string(),
                    phone: Some("9123249740".to_string()),
                    mail: Some("2216138@sliet.ac.in".to_string()),
                    roll_no: Some("2216138".to_string()),
                    socials: None,
                },
                TeamMember {
                    name: "Asmit Ayank".to_string(),
                    rank: None,
                    deptt: Deptt::CDE,
                    photo: "/images/ASMIT AYANK.jpg".to_string(),
                    phone: Some("8104540350".to_string()),
                    mail: Some("2110234@sliet.ac.in".to_string()),
                    roll_no: Some("2110234".to_string()),
                    socials: Some("https://www.linkedin.com/in/asmit-ayank-b45696272".to_string()),
                },
                TeamMember {
                    name: "Shashank Shekhar Trivedi".to_string(),
                    rank: None,
                    deptt: Deptt::CDE,
                    photo: "/images/SHASHANK SHEKHAR TRIVEDI.jpg".to_string(),
                    phone: Some("9999190445".to_string()),
                    mail: Some("2110425@sliet.ac.in".to_string()),
                    roll_no: Some("2110425".to_string()),
                    socials: Some("https://www.linkedin.com/in/shashank-shekhar-trivedi-136777270".to_string()),
                },
                TeamMember {
                    name: "Riya Kumari".to_string(),
                    rank: None,
                    deptt: Deptt::CDE,
                    photo: "/images/RIYA KUMARI.jpg".to_string(),
                    phone: None,
                    mail: Some("2211071@sliet.ac.in".to_string()),
                    roll_no: Some("2211071".to_string()),
                    socials: Some("linkedin.com/in/riya-kumari".to_string()),
                },
                TeamMember {
                    name: "Sandeep Kumar".to_string(),
                    rank: Some(Rank::WebDev),
                    deptt: Deptt::GCS,
                    photo: "/images/Sandeep Kumar.jpg".to_string(),
                    phone: Some("7807113002".to_string()),
                    mail: Some("sandeepqmar05@gmail.com".to_string()),
                    roll_no: Some("2241028".to_string()),
                    socials: Some("https://www.linkedin.com/in/sandeep-kumar-746095257".to_string()),
                },
                TeamMember {
                    name: "Srinjoy Chakraborty".to_string(),
                    rank: None,
                    deptt: Deptt::GEE,
                    photo: "/images/SRINJOY CHAKRABORTY.jpg".to_string(),
                    phone: Some("9800572891".to_string()),
                    mail: Some("2140301@sliet.ac.in".to_string()),
                    roll_no: Some("2140301".to_string()),
                    socials: Some("https://www.linkedin.com/in/srinjoysliet".to_string()),
                },
                TeamMember {
                    name: "Kaushal Sharma".to_string(),
                    rank: None,
                    deptt: Deptt::GCS,
                    photo: "/images/KUSHAL SHARMA.jpg".to_string(),
                    phone: Some("8630829288".to_string()),
                    mail: None,
                    roll_no: Some("2140159".to_string()),
                    socials: Some("https://www.linkedin.com/in/itskushal0403".to_string()),
                },
                TeamMember {
                    name: "Amit Kumar".to_string(),
                    rank: None,
                    deptt: Deptt::GEE,
                    photo: "/images/AMIT KUMAR.jpg".to_string(),
                    phone: Some("9828243511".to_string()),
                    mail: Some("2140315@sliet.ac.in".to_string()),
                    roll_no: Some("2140215".to_string()),
                    socials: Some("https://linkedin.com/in/Amit Kumar".to_string()),
                },
                TeamMember {
                    name: "Pappu Kumar".to_string(),
                    rank: None,
                    deptt: Deptt::GEE,
                    photo: "/images/PAPPU KUMAR.jpg".to_string(),
                    phone: Some("9693560261".to_string()),
                    mail: Some("2234116@sliet.ac.in".to_string()),
                    roll_no: Some("2234116".to_string()),
                    socials: Some("https://www.linkedin.com/in/pappu-kumar-bb669327b".to_string()),
                },
            ],
        ),
    ]);
    let currently_selected = create_rw_signal(cx, Some(TeamSections::Core));
    let (teams_view_getter, teams_view_setter) = create_signal(cx, vec![]);
    let (teams_signal_getter, teams_signal_setter) = create_signal(cx, vec![]);
    create_effect(cx, move |_| {
        if let Some(currently_selected) = currently_selected.get() {
            let current_projects = &content[&currently_selected];
            let mut members_displaying = teams_view_getter.with_untracked(|p| p.len());
            while members_displaying > content[&currently_selected].len() {
                members_displaying -= 1;
                set_timeout(
                    move || {
                        teams_view_setter.update(|projects| {
                            projects.pop();
                        })
                    },
                    Duration::from_millis(1000) ,
                );
                set_timeout(
                    move || {
                        teams_signal_setter.update(|projects: &mut Vec<TeamMemberState>| {
                            match projects.pop() {
                                Some(project) => project.is_hidden.set(true),
                                None => {}
                            };
                        })
                    },
                    Duration::from_millis(1000),
                );
            }
            while members_displaying < content[&currently_selected].len() {
                members_displaying += 1;
                teams_view_setter.update(|teams_view| {
                    let (photo, name, deptt, mail, rank, phone, socials, roll_no) = (
                        create_rw_signal(cx, current_projects[teams_view.len()].photo.clone()),
                        create_rw_signal(cx, current_projects[teams_view.len()].name.clone()),
                        create_rw_signal(cx, current_projects[teams_view.len()].deptt.clone()),
                        create_rw_signal(cx, current_projects[teams_view.len()].mail.clone()),
                        create_rw_signal(cx, current_projects[teams_view.len()].rank.clone()),
                        create_rw_signal(cx, current_projects[teams_view.len()].phone.clone()),
                        create_rw_signal(cx, current_projects[teams_view.len()].socials.clone()),
                        create_rw_signal(cx, current_projects[teams_view.len()].roll_no.clone()),
                    );
                    let is_hidden = create_rw_signal(cx, true);
                    set_timeout(
                        move || {
                            is_hidden.set(false);
                        },
                        Duration::from_millis(1000),
                    );
                    teams_view.push(view! {cx,
                        <div class="project_member" class:hidden=move || is_hidden.get()>
                            <img src={move || photo.get()} class="project_image"/>
                            <div class="member_info">
                                <div class="member_name">
                                    <div class="label">
                                        Name
                                    </div>
                                    <div class="data">
                                        {name}
                                    </div>
                                </div>
                                { move || if phone.get().is_some() {
                                    view!{cx, 
                                        <div class="member_phone">
                                            <div class="label">
                                            Phone No.
                                            </div>
                                            <div class="data">
                                            {phone}
                                            </div>
                                        </div>
                                    }.into_view(cx)
                                } else {
                                        view!{cx, }.into_view(cx)
                                    }
                                }
                                { move || if roll_no.get().is_some() {
                                    view!{cx, 
                                        <div class="member_roll_no">
                                            <div class="label">
                                            Roll No.
                                            </div>
                                            <div class="data">
                                            {roll_no}
                                            </div>
                                        </div>
                                    }.into_view(cx)
                                } else {
                                        view!{cx, }.into_view(cx)
                                    }
                                }
                                <div class="member_deptt">
                                    <div class="label">
                                        Deptt.
                                    </div>
                                    <div class="data">
                                        {move || deptt.get().to_string()}
                                    </div>
                                </div>
                                { move || if mail.get().is_some() {
                                    view!{cx, 
                                        <div class="member_mail">
                                            <div class="label">
                                            E-Mail
                                            </div>
                                            <div class="data">
                                            {mail}
                                            </div>
                                        </div>
                                    }.into_view(cx)
                                } else {
                                        view!{cx, }.into_view(cx)
                                    }
                                }
                        {
                            move || match rank.get() {
                                Some(s) => view!{
                                    cx, 
                                    <div class="member_rank">
                                        {s.to_string()}
                                    </div>
                                }.into_view(cx),
                                None => view!{cx, }.into_view(cx),
                            }
                        }

                                { move || if socials.get().is_some() {
                                    view!{cx, 
                                        <div class="linkedin">
                                            <a href=socials>
                                            <img width=28 height=28 src="https://upload.wikimedia.org/wikipedia/commons/8/81/LinkedIn_icon.svg"/>
                                            </a>
                                        </div>
                                    }.into_view(cx)
                                } else {
                                        view!{cx, }.into_view(cx)
                                    }
                                }
                            </div>
                    </div>});
                    teams_signal_setter.update(|signals| {
                        signals.push(TeamMemberState {
                            photo,
                            name,
                            phone,
                            roll_no,
                            deptt,
                            mail,
                            rank,
                            socials,
                            is_hidden,
                        })
                    })
                });
            }
            if members_displaying == content[&currently_selected].len() {
                teams_signal_getter.with_untracked(|members_signal| {
                    for (
                        TeamMemberState {
                            name,
                            phone,
                            roll_no,
                            deptt,
                            mail,
                            rank,
                            is_hidden: _,
                            photo,
                            socials,
                        },
                        member,
                    ) in members_signal
                        .into_iter()
                        .zip(content[&currently_selected].iter())
                    {
                        name.set(member.name.clone());
                        phone.set(member.phone.clone());
                        roll_no.set(member.roll_no.clone());
                        deptt.set(member.deptt.clone());
                        mail.set(member.mail.clone());
                        rank.set(member.rank.clone());
                        photo.set(member.photo.clone());
                        socials.set(member.socials.clone());
                    }
                });
            }
        }
    });
    view! {cx,
        <div class="heading">
            "Our Team"
        </div>
        <div class="selector_btns" class:selected=move || currently_selected.get().is_none()>
            <EnumButtons currently_selected={currently_selected.into()}/>
        </div>
                <div class="hidden"
                    class:hidden=move || currently_selected.get().is_none() class="content" class:faculty_selected=move || match currently_selected.get() {
                    Some(TeamSections::Faculty) => true,
                    _ => false,
                }>
        <div class="members_list">
    {move || {
        match currently_selected.get() {
            Some(_) => {teams_view_getter.get()},
            None => {Vec::new()},
        }
    }}
        </div>
    </div>
    }
}

#[component]
fn Gallery(cx: Scope) -> impl IntoView {
    let images = [("/images/IMG-20230207-WA0150.jpg", "gallery_image"),
        ("/images/IMG-20230207-WA0151.jpg", "gallery_image"),
        ("/images/IMG-20230207-WA0152.jpg", "gallery_image"),
        ("/images/IMG-20230207-WA0153.jpg", "gallery_image"),
        ("/images/IMG-20230207-WA0159.jpg", "gallery_image"),
        ("/images/IMG-20230207-WA0160.jpg", "gallery_image"),
        ("/images/IMG-20230207-WA0161.jpg", "gallery_image"),
        ("/images/IMG-20230207-WA0162.jpg", "gallery_image"),
        ("/images/IMG-20230207-WA0163.jpg", "gallery_image"),
        ("/images/IMG-20230207-WA0165.jpg", "gallery_image"),
        ("/images/IMG-20230207-WA0166.jpg", "gallery_image"),
        ("/images/IMG-20230207-WA0167.jpg", "gallery_image"),
        ("/images/IMG-20230212-WA0007.jpg", "gallery_image"),
        ("/images/IMG-20230328-WA0032.jpg", "gallery_image"),
        ("/images/IMG-20230328-WA0033.jpg", "gallery_image"),
        ("/images/IMG-20230401-WA0000.jpg", "gallery_image"),
        ("/images/IMG-20230401-WA0001.jpg", "gallery_image"),
        ("/images/IMG-20230401-WA0014.jpg", "gallery_image"),
        ("/images/IMG-20230401-WA0015.jpg", "gallery_image"),
        ("/images/IMG-20230401-WA0016.jpg", "gallery_image"),
        ("/images/IMG-20230401-WA0025.jpg", "gallery_image"),
        ("/images/IMG-20230401-WA0026.jpg", "gallery_image"),
        ("/images/IMG-20230401-WA0027.jpg", "gallery_image"),
        ("/images/IMG-20230511-WA0007.jpg", "gallery_image"),
        ("/images/IMG_20230511_180813_939.jpg", "gallery_image"),
        ("/images/IMG_20230511_180827_256.jpg", "gallery_image"),
        ("/images/IMG_20230511_180827_571.jpg", "gallery_image"),
        ("/images/IMG_20230511_180846_738.jpg", "gallery_image"),
        ("/images/IMG_20230511_180924_651.jpg", "gallery_image"),
        ("/images/IMG_20230511_180949_446.jpg", "gallery_image"),
        ("/images/IMG_20230511_180950_073.jpg", "gallery_image"),
        ("/images/IMG_20230511_183546_052.jpg", "gallery_image"),
        ("/images/IMG_20230511_183554_882.jpg", "gallery_image"),
        ("/images/IMG_20230511_183627_122.jpg", "gallery_image"),
    ];
    const ROWS: usize = 3;
    let gallery_view = images
        .into_iter()
        .map(|(image_path, image_desc)| {
            view! {
                cx, <img alt={image_desc} src={image_path}/>
            }
        })
        .collect_view(cx);
    view! {
        cx,
        <div class="heading">
            "Gallery"
            </div>
            <div id="photos">
            {gallery_view}
            </div>
    }
}

#[derive(Debug, PartialEq, Eq, Sequence, Clone, Copy, Hash)]
enum AboutSection {
    Aim,
    Establishment,
    Objectives,
    Achievements,
    StudentIdeas,
}
impl ToString for AboutSection {
    fn to_string(&self) -> String {
        match self {
            Self::Aim => "Aim".to_string(),
            Self::Establishment => "Establishment".to_string(),
            Self::Objectives => "Objectives".to_string(),
            Self::Achievements => "Achievements".to_string(),
            Self::StudentIdeas => "Student Ideas".to_string(),
        }
    }
}

#[component]
fn About(cx: Scope) -> impl IntoView {
    let content_data = HashMap::from([
        (AboutSection::Establishment, vec![
            "Mavericks Club, SLIET was established by Dhurv Patel (GEE/2015). in 2017".to_string()]),

        (AboutSection::Aim, vec![
            "Offering an excellent opportunity for students with backgrounds in Mechanical, Mechatronics, Electronics, Computers, and Electrical engineering to enhance their skills and demonstrate their talent through a wide range of interdisciplinary activities.".to_string()]),

        (AboutSection::Objectives, vec![
            "To promote creativity and to increase the technical now – how and productivity of all the students at the college.".to_string(),
            "To promote hands -on and co-operative learning and also engages students in problem solving higher order thinking.".to_string(),
            "To develop the spirit of entrepreneurship among the students of institute.".to_string()]),

        (AboutSection::Achievements, vec![
            "Won in Smart India Hackathon (SIH) in 2019 & 2020.".to_string(),
            "Two Team qualified for the zonal rounds of DRDO'S Unmanned Vehicle Challenge.".to_string(), 
            "Among the top teams in E-Yantra 2017, 2018, 2019 Competition organised by { MOE } in accordance with IIT Bombay.".to_string(), 
            "Spreaded glories at various technical fest, including IIT Bombay, IIT Kanpur, IIT Delhi, IIT Bhu, IIT Ropar, PEC, Thapar University and recevied cash prize at Guru Nanak Dev University , Amritsar Punjab.".to_string(),
            "In our team, we provide guidance for developing skills in various Coding Platforms, Development Board, Cricut Design Software, 3D Modelling Software, Hardware Implementation and the latest technologies.".to_string()]),

        (AboutSection::StudentIdeas, vec![
            "Prashant Sharma (Drone Ninja) : The purpose of this idea is to make a drone that can help our farmers in farming.".to_string(),
            "Ashish Bobby (Smart Irrigation) : Making an automated irrigation system that can judiciously use water for irrigation.".to_string()
        ]),
    ]);

    let currently_selected = create_rw_signal(cx, Some(AboutSection::Aim));
    let (current_text_getter, current_text_setter) = create_signal(cx, Vec::new());
    let (is_transitioning, transition) = create_signal(cx, false);
    create_effect(cx, move |_| {
        if let Some(currently_selected) = currently_selected.get() {
            let current_texts = content_data[&currently_selected].clone();
            transition(true);
            set_timeout(
                move || {
                    current_text_setter.update(|t| t.clear());
                    for line in current_texts.into_iter() {
                        current_text_setter.update(|t| t.push(view! {cx, <li>{line.clone()}</li>}));
                        transition(false);
                    }
                },
                Duration::from_millis(750),
            );
        }
    });
    view! {cx,
        <div class="heading">
            "About Us"
        </div>
        <div class="selector_btns">
            <EnumButtons currently_selected=currently_selected/>
        </div>
        <ul class="content" class:hidden=move || is_transitioning()>{move || current_text_getter.get()}</ul>
    }
}

#[component]
fn Projects(cx: Scope) -> impl IntoView {
    let projects = vec![
        Project {
            name: "Mini Hovercraft".to_string(),
            image: "/images/miniHover.jpg".to_string(),
            desc: "A hovercraft, also known as an air-cushion vehicle or ACV, is an amphibious craft capable of travelling over land, water, mud, ice, and other surfaces.".to_string(),
            doc: "/docs/MINI HOVERCRAFT.docx".to_string(),

        },
        Project {
            name: "Electromate".to_string(),
            image: "/images/Electromate.jpg".to_string(),
            desc: "In the heart of our college's techfest, a competition named Electromate set the stage for an inspiring display of technological prowess and empathy. The challenge was to devise a solution that could enhance the mobility and safety of visually impaired individuals. As part of the robotics club, we rose to the occasion and designed a smart stick that not only helped the blind navigate obstacles but also addressed other critical challenges they face.".to_string(),
            doc: "/docs/ElectromateReport.docx".to_string(),
        },
        Project {
            name: "3D Printer".to_string(),
            image: "/images/3D Printer.jpg".to_string(),
            desc: "A 3-d printer is machine with the help of which we build real physical object from 3-D digital design by putting thin layers of material.".to_string(),
            doc: "/docs/Project information.docx".to_string(),
        },
        Project {
            name: "Soccer Bot".to_string(),
            image: "/images/Soccer Bot.png".to_string(),
            desc: "A soccer bot using a Bluetooth module is a robotic device designed to play soccer or engage in soccer-related activities.".to_string(),
            doc: "/docs/SOCCER BOT.docx".to_string(),
        },
    ];
    let projects_view = projects.into_iter().map(|project| {
        view! {cx,
            <div class="project_item">
                <img src=project.image class="project_image"/>
                <div class="project_name">
                    {project.name}
                </div>
                <div class="project_desc">
                    {project.desc}
                </div>
                <a href={project.doc}>
                    <img src="/images/doc.svg" style="filter: invert(1);" />
                </a>
            </div>}
    }).collect_view(cx);
    view! {cx,
        <div class="heading">
            "Projects"
        </div>
        <div class="content">
            <div class="projects_list">
                {projects_view}
            </div>
        </div>
    }
}

#[component]
fn EnumButtons<T>(cx: Scope, currently_selected: RwSignal<Option<T>>) -> impl IntoView
where
    T: 'static + Sequence + ToString + PartialEq + Eq + Clone + Hash + Copy,
{
    all::<T>()
        .map(|e| {
            view! {cx, <button class:button_selected=move || {
                if let Some(selected) = currently_selected.get() {
                    selected == e
                } else {
                    false
                }
            } on:click=move |_| {
                currently_selected.set(Some(e));
                }> {e.to_string()}</button>}
        })
        .collect_view(cx)
}

#[derive(Debug, PartialEq, Eq, Sequence, Clone, Copy, Hash)]
enum Domain {
    Mechanical,
    Electrical,
    Electronics,
}

impl Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mechanical => f.write_str("Mechanical"),
            Self::Electronics => f.write_str("Electronics"),
            Self::Electrical => f.write_str("Electrical"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Project {
    name: String,
    image: String,
    desc: String,
    doc: String,
}
#[component]
fn Landing(cx: Scope) -> impl IntoView {
    let (mottos, _) = create_signal(
        cx,
        vec![
            "Different".to_string(),
            "Inspiring".to_string(),
            "Engineers".to_string(),
        ],
    );

    view! {cx,
        <div class="landing">
            <div class="heading">
            "Team Mavericks"
                <div class="motto">
                "Born to be " <ChangingText 
                    text={mottos.into()} 
                    time_per_letter={Duration::from_millis(100)} 
                    time_per_word={Duration::from_secs(3)}/>
                </div>
            </div>
            <div class="cta">
                <div class="heading"> Explore </div>
                    {all::<Routes>()
                        .map(|dest| 
                            view! {cx, 
                                <A href={dest.path()}>
                                    <div>
                                        {dest.to_string()}
                                    </div>
                                </A>})
                        .collect_view(cx)}
            </div>
        </div>
    }
}

#[component]
fn ChangingText(
    cx: Scope,
    text: Signal<Vec<String>>,
    time_per_letter: Duration,
    time_per_word: Duration,
) -> impl IntoView {
    let (text_getter, text_setter) = create_signal(cx, text.with_untracked(|text| text[0].clone()));
    let (text_idx, set_text_number) = create_signal(cx, 0);
    set_interval(
        move || {
            set_text_number.update(|x| *x = (*x + 1) % text.with(|text| text.len()));
            text_setter(text.with(|texts| texts[text_idx()].clone()));
        },
        time_per_word,
    );
    view! {cx,
        <TransitioningText time_per_letter=time_per_letter initial={text.with_untracked(|text| text[0].clone())} text={text_getter.into()} />
    }
}

#[component]
fn TransitioningText(
    cx: Scope,
    initial: String,
    text: Signal<String>,
    time_per_letter: Duration,
) -> impl IntoView {
    let (text_getter, text_setter) = create_signal(cx, initial);
    create_effect(cx, move |old_state: Option<RwSignal<bool>>| {
        let stale_state = create_rw_signal(cx, false);
        let t = text.get();
        if let Some(old_stale_state) = old_state {
            old_stale_state.set(true);
        }
        let initial_length = text_getter.with_untracked(|t| t.len());
        for i in 0..initial_length {
            set_timeout(
                move || {
                    if stale_state.get_untracked() {
                        return;
                    }
                    text_setter.update(|t| {
                        t.pop();
                    })
                },
                time_per_letter * i as u32,
            );
        }
        for (i, c) in t.chars().enumerate() {
            set_timeout(
                move || {
                    if stale_state.get_untracked() {
                        return;
                    }
                    text_setter.update(|t| {
                        t.push(c);
                    })
                },
                time_per_letter * (i + initial_length) as u32,
            );
        }
        stale_state
    });
    text_getter
}

#[component]
fn Navbar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="navbar">
            <img src="/images/logo.svg"/>
            <div class="navbar_buttons">
            {all::<Routes>().map(|dest| view! {cx, <A href={dest.path()}><div>{dest.to_string()}</div></A>} ).collect_view(cx)}
            </div>
        </div>
    }
}

#[derive(Debug, PartialEq, Eq, Sequence, Clone, Copy, Hash)]
enum Routes {
    Home,
    Projects,
    About,
    Gallery,
}

impl ToString for Routes {
    fn to_string(&self) -> String {
        match self {
            Self::Home => "Home".to_string(),
            Self::About => "About".to_string(),
            Self::Gallery => "Gallery".to_string(),
            Self::Projects => "Projects".to_string(),
        }
    }
}

impl Routes {
    fn path(&self) -> String {
        match self {
            Self::Home => "/".to_string(),
            Self::About => "/about".to_string(),
            Self::Gallery => "/gallery".to_string(),
            Self::Projects => "/projects".to_string(),
        }
    }
}
#[component]
fn Page(cx: Scope, id: &'static str, children: Children) -> impl IntoView {
    view! { cx,
       <div class="page" id={id}>
            {children(cx)}
            </div>
    }
}
