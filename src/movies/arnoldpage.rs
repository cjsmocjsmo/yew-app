use yew::prelude::*;
use stylist::yew::use_style;





#[function_component]
pub fn ArnoldImageGroup() -> Html {
    struct Video {
        catagory: String,
        dirpath: String,
        filepath: String,
        genre: String,
        httpthumbpath: String,
        mediaid: String,
        movfspath: String,
        movname: String,
        movyear: String,
        thumbpath: String,
    }
    let videos: Vec<Video> = vec![
        Video {
            catagory:"Arnold".to_string(),
            dirpath:"/root/fsData/Movies/Arnold/".to_string(),
            filepath:"/root/fsData/Movies/Arnold/Conan The Barbarian (1982).mp4".to_string(),
            genre:"movies".to_string(),
            httpthumbpath:"http://192.168.0.94:8888/static/Conan The Barbarian (1982).jpg".to_string(),
            mediaid:"419009239769702206".to_string(),
            movfspath:"/media/pi/PiTB/media/Movies/Arnold/Conan The Barbarian (1982).mp4".to_string(),
            movname:"ConanTheBarbarian".to_string(),
            movyear:"1982".to_string(),
            thumbpath:"./static/Conan The Barbarian (1982).jpg".to_string()
        },
        Video {
            catagory:"Arnold".to_string(),
            dirpath:"/root/fsData/Movies/Arnold/".to_string(),
            filepath:"/root/fsData/Movies/Arnold/Conan The Destroyer (1984).mp4".to_string(),
            genre:"movies".to_string(),
            httpthumbpath:"http://192.168.0.94:8888/static/Conan The Destroyer (1984).jpg".to_string(),
            mediaid:"1138150044767549429".to_string(),
            movfspath:"/media/pi/PiTB/media/Movies/Arnold/Conan The Destroyer (1984).mp4".to_string(),
            movname:"ConanTheDestroyer".to_string(),
            movyear:"1984".to_string(),
            thumbpath:"./static/Conan The Destroyer (1984).jpg".to_string()
        },

        Video {
            catagory:"Arnold".to_string(),
            dirpath:"/root/fsData/Movies/Arnold/".to_string(),
            filepath:"/root/fsData/Movies/Arnold/Predator (1987).mp4".to_string(),
            genre:"movies".to_string(),
            httpthumbpath:"http://192.168.0.94:8888/static/Predator (1987).jpg".to_string(),
            mediaid:"1383007431172979982".to_string(),
            movfspath:"/media/pi/PiTB/media/Movies/Arnold/Predator (1987).mp4".to_string(),
            movname:"Predator".to_string(),
            movyear:"1987".to_string(),
            thumbpath:"./static/Predator (1987).jpg".to_string()
        },

        Video {
            catagory:"Arnold".to_string(),
            dirpath:"/root/fsData/Movies/Arnold/".to_string(),
            filepath:"/root/fsData/Movies/Arnold/Predator 2 (1990).mp4".to_string(),
            genre:"movies".to_string(),
            httpthumbpath:"http://192.168.0.94:8888/static/Predator 2 (1990).jpg".to_string(),
            mediaid:"1501173847728789245".to_string(),
            movfspath:"/media/pi/PiTB/media/Movies/Arnold/Predator 2 (1990).mp4".to_string(),
            movname:"Predator2".to_string(),
            movyear:"1990".to_string(),
            thumbpath:"./static/Predator 2 (1990).jpg".to_string()
        },

        Video {
            catagory:"Arnold".to_string(),
            dirpath:"/root/fsData/Movies/Arnold/".to_string(),
            filepath:"/root/fsData/Movies/Arnold/Terminator 2 (1991).mp4".to_string(),
            genre:"movies".to_string(),
            httpthumbpath:"http://192.168.0.94:8888/static/Terminator 2 (1991).jpg".to_string(),
            mediaid:"1210495984823524812".to_string(),
            movfspath:"/media/pi/PiTB/media/Movies/Arnold/Terminator 2 (1991).mp4".to_string(),
            movname:"Terminator2".to_string(),
            movyear:"1991".to_string(),
            thumbpath:"./static/Terminator 2 (1991).jpg".to_string()
        },

        Video {
            catagory:"Arnold".to_string(),
            dirpath:"/root/fsData/Movies/Arnold/".to_string(),
            filepath:"/root/fsData/Movies/Arnold/Terminator 3 Rise Of The Machines (2003).mp4".to_string(),
            genre:"movies".to_string(),
            httpthumbpath:"http://192.168.0.94:8888/static/Terminator 3 Rise Of The Machines (2003).jpg".to_string(),
            mediaid:"1380983643298219800".to_string(),
            movfspath:"/media/pi/PiTB/media/Movies/Arnold/Terminator 3 Rise Of The Machines (2003).mp4".to_string(),
            movname:"Terminator3RiseOfTheMachines".to_string(),
            movyear:"2003".to_string(),
            thumbpath:"./static/Terminator 3 Rise Of The Machines (2003).jpg".to_string()
        },
        Video {
            catagory:"Arnold".to_string(),
            dirpath:"/root/fsData/Movies/Arnold/".to_string(),
            filepath:"/root/fsData/Movies/Arnold/Terminator Genisys (2015).mp4".to_string(),
            genre:"movies".to_string(),
            httpthumbpath:"http://192.168.0.94:8888/static/Terminator Genisys (2015).jpg".to_string(),
            mediaid:"809048412998838846".to_string(),
            movfspath:"/media/pi/PiTB/media/Movies/Arnold/Terminator Genisys (2015).mp4".to_string(),
            movname:"TerminatorGenisys".to_string(),
            movyear:"2015".to_string(),
            thumbpath:"./static/Terminator Genisys (2015).jpg".to_string()
        },

        Video {
            catagory:"Arnold".to_string(),
            dirpath:"/root/fsData/Movies/Arnold/".to_string(),
            filepath:"/root/fsData/Movies/Arnold/Terminator Salvation (2009).mp4".to_string(),
            genre:"movies".to_string(),
            httpthumbpath:"http://192.168.0.94:8888/static/Terminator Salvation (2009).jpg".to_string(),
            mediaid:"1259787622528254581".to_string(),
            movfspath:"/media/pi/PiTB/media/Movies/Arnold/Terminator Salvation (2009).mp4".to_string(),
            movname:"TerminatorSalvation".to_string(),
            movyear:"2009".to_string(),
            thumbpath:"./static/Terminator Salvation (2009).jpg".to_string()
        },

        Video {
            catagory:"Arnold".to_string(),
            dirpath:"/root/fsData/Movies/Arnold/".to_string(),
            filepath:"/root/fsData/Movies/Arnold/The Last Stand (2013).mp4".to_string(),
            genre:"movies".to_string(),
            httpthumbpath:"http://192.168.0.94:8888/static/The Last Stand (2013).jpg".to_string(),
            mediaid:"1174829929286162208".to_string(),
            movfspath:"/media/pi/PiTB/media/Movies/Arnold/The Last Stand (2013).mp4".to_string(),
            movname:"TheLastStand".to_string(),
            movyear:"2013".to_string(),
            thumbpath:"./static/The Last Stand (2013).jpg".to_string()
        },

        Video {
            catagory:"Arnold".to_string(),
            dirpath:"/root/fsData/Movies/Arnold/".to_string(),
            filepath:"/root/fsData/Movies/Arnold/The Terminator (1984).mp4".to_string(),
            genre:"movies".to_string(),
            httpthumbpath:"http://192.168.0.94:8888/static/The Terminator (1984).jpg".to_string(),
            mediaid:"271291280623831677".to_string(),
            movfspath:"/media/pi/PiTB/media/Movies/Arnold/The Terminator (1984).mp4".to_string(),
            movname:"TheTerminator".to_string(),
            movyear:"1984".to_string(),
            thumbpath:"./static/The Terminator (1984).jpg".to_string()
        },

        Video {
            catagory:"Arnold".to_string(),
            dirpath:"/root/fsData/Movies/Arnold/".to_string(),
            filepath:"/root/fsData/Movies/Arnold/Total Recall EXTENDED (2012).mp4".to_string(),
            genre:"movies".to_string(),
            httpthumbpath:"http://192.168.0.94:8888/static/Total Recall EXTENDED (2012).jpg".to_string(),
            mediaid:"1447232428068823085".to_string(),
            movfspath:"/media/pi/PiTB/media/Movies/Arnold/Total Recall EXTENDED (2012).mp4".to_string(),
            movname:"TotalRecallEXTENDED".to_string(),
            movyear:"2012".to_string(),
            thumbpath:"./static/Total Recall EXTENDED (2012).jpg".to_string()
        },
        Video {
            catagory:"Arnold".to_string(),
            dirpath:"/root/fsData/Movies/Arnold/".to_string(),
            filepath:"/root/fsData/Movies/Arnold/Total Recall Mind Bending Edition (1990).mp4".to_string(),
            genre:"movies".to_string(),
            httpthumbpath:"http://192.168.0.94:8888/static/Total Recall Mind Bending Edition (1990).jpg".to_string(),
            mediaid:"1058926325942486892".to_string(),
            movfspath:"/media/pi/PiTB/media/Movies/Arnold/Total Recall Mind Bending Edition (1990).mp4".to_string(),
            movname:"TotalRecallMindBendingEdition".to_string(),
            movyear:"1990".to_string(),
            thumbpath:"./static/Total Recall Mind Bending Edition (1990).jpg".to_string()
        },

        Video {
            catagory:"Arnold".to_string(),
            dirpath:"/root/fsData/Movies/Arnold/".to_string(),
            filepath:"/root/fsData/Movies/Arnold/True Lies (1994).mp4".to_string(),
            genre:"movies".to_string(),
            httpthumbpath:"http://192.168.0.94:8888/static/True Lies (1994).jpg".to_string(),
            mediaid:"357068898200537814".to_string(),
            movfspath:"/media/pi/PiTB/media/Movies/Arnold/True Lies (1994).mp4".to_string(),
            movname:"TrueLies".to_string(),
            movyear:"1994".to_string(),
            thumbpath:"./static/True Lies (1994).jpg".to_string()
        }
    ];

    let img_style = use_style!("
        display: block;
        margin-top: 10px;
        margin-right: auto;
        margin-left: auto;
        width: 200px;
        border-radius: 8px;
    ");
    let img_div = use_style!("
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: center;
        align-items: center;
        margin-bottom: 60px;
    ");
    let videos = videos.iter().map(|video| html! {
        <img
                    
                    id={video.mediaid.clone()}
                    key={video.mediaid.clone()} 
                    class={img_style.clone()} 
                    src={video.httpthumbpath.clone()} 
                    alt="video thumbnail" 
                />
    }).collect::<Html>();
html!(
    <>
        <div class={ img_div }>
            { videos }
        </div>
    </>
)
}




#[function_component]
pub fn ArnoldPage() -> Html {
    

    
    
    let h1_style = use_style!("
        text-align: center;
        color: #ebb917;
    ");

    html!(
        <>
            <crate::comps::mainpage_comps::MyHeader />
            <h1 class={h1_style}>{"Arnold"}</h1>
            <ArnoldImageGroup />
            <crate::comps::mainpage_comps::ImageGroupTwo />
            <crate::comps::mainpage_comps::PlayerControls />
        </>
    )
}