use yew::prelude::*;

#[function_component(Intro)]
pub fn intro() -> Html { 
    html! {
        <div class="w-full flex flex-row justify-between mt-10">
            <div class="w-8/12">
                <div class="w-9/12">
                   <h1 class="text-5xl uppercase font-bold">{"Fullstack Software Developer"}</h1> 
               </div>
            </div>
            <div class="w-4/12 flex flex-row justify-end items-end">
                <a class="w-10 h-10 flex items-center justify-center 
                transition duration-200 ease-in-out
                border border-gray-800 rounded-full hover:bg-gray-800 hover:text-white" href="https://instagram.com/sk.developer">
                    <i class="fa fa-instagram" aria-hidden="true"></i>
                </a>
                <a class="w-10 h-10 flex items-center justify-center
                transition duration-200 ease-in-out
                border border-gray-800 rounded-full hover:bg-gray-800 hover:text-white
                rounded-full ml-2" href="https://twitter.com/saeedkdev">
                    <i class="fa fa-twitter" aria-hidden="true"></i>
                </a>
                <a class="w-10 h-10 flex items-center justify-center
                transition duration-200 ease-in-out
                border border-gray-800 rounded-full hover:bg-gray-800 hover:text-white
                rounded-full ml-2" href="https://linkedin.com/in/saeed-k">
                    <i class="fa fa-linkedin" aria-hidden="true"></i>
                </a>
            </div>
        </div>
    }
}
