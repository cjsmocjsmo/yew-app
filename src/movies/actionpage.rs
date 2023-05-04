// use yew_router::prelude::*;
use yew::prelude::*;
use stylist::yew::use_style;





#[function_component]
pub fn ImageGroupThree() -> Html {
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

    let videos = vec![ 
    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Alita Battle Angel (2019).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Alita Battle Angel (2019).jpg".to_string(),
        mediaid:"1237421983530919876".to_string(),
        movfspath:"/media/pi/PiTB/media/Movies/Action/Alita Battle Angel (2019).mp4".to_string(),
        movname:"AlitaBattleAngel".to_string(),
        movyear:"2019".to_string(),
        thumbpath:"./static/Alita Battle Angel (2019).jpg".to_string()
    },
    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/American Assassin (2017).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/American Assassin (2017).jpg".to_string(), 
        mediaid:"1279789958612294916".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/American Assassin (2017).mp4".to_string(),
        movname:"AmericanAssassin".to_string(),
        movyear:"2017".to_string(),
        thumbpath:"./static/American Assassin (2017).jpg".to_string()
    },
    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Apocalypse Now (1979).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Apocalypse Now (1979).jpg".to_string(),
        mediaid:"1230500701332561871".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Apocalypse Now (1979).mp4".to_string(),
        movname:"ApocalypseNow".to_string(),
        movyear:"1979".to_string(),
        thumbpath:"./static/Apocalypse Now (1979).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Army Of One (2020).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Army Of One (2020).jpg".to_string(),
        mediaid:"1052970277590116243".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Army Of One (2020).mp4".to_string(),
        movname:"ArmyOfOne".to_string(),movyear:"2020".to_string(),
        thumbpath:"./static/Army Of One (2020).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Assassins Creed (2016).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Assassins Creed (2016).jpg".to_string(),
        mediaid:"1393758420165356815".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Assassins Creed (2016).mp4".to_string(),
        movname:"AssassinsCreed".to_string(),
        movyear:"2016".to_string(),
        thumbpath:"./static/Assassins Creed (2016).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Atomic Blonde (2017).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Atomic Blonde (2017).jpg".to_string(),
        mediaid:"324379747414679440".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Atomic Blonde (2017).mp4".to_string(),
        movname:"AtomicBlonde".to_string(),
        movyear:"2017".to_string(),
        thumbpath:"./static/Atomic Blonde (2017).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Baby Driver (2017).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Baby Driver (2017).jpg".to_string(),
        mediaid:"856409653515388981".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Baby Driver (2017).mp4".to_string(),
        movname:"BabyDriver".to_string(),
        movyear:"2017".to_string(),
        thumbpath:"./static/Baby Driver (2017).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Ben Hur (1959).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Ben Hur (1959).jpg".to_string(),
        mediaid:"215224404002380281".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Ben Hur (1959).mp4".to_string(),
        movname:"BenHur".to_string(),
        movyear:"1959".to_string(),
        thumbpath:"./static/Ben Hur (1959).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Everest (2015).avi".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Everest (2015).jpg".to_string(),
        mediaid:"434089176103716594".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Everest (2015).avi".to_string(),
        movname:"Everest".to_string(),
        movyear:"2015".to_string(),
        thumbpath:"./static/Everest (2015).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Everything Everywhere All At Once (2022).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Everything Everywhere All At Once (2022).jpg".to_string(),
        mediaid:"805570100415182536".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Everything Everywhere All At Once (2022).mp4".to_string(),
        movname:"EverythingEverywhereAllAtOnce".to_string(),
        movyear:"2022".to_string(),
        thumbpath:"./static/Everything Everywhere All At Once (2022).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Exposed (2016).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Exposed (2016).jpg".to_string(),
        mediaid:"31191560142863993".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Exposed (2016).mp4".to_string(),
        movname:"Exposed".to_string(),
        movyear:"2016".to_string(),
        thumbpath:"./static/Exposed (2016).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Fistful Of Vengeance (2022).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Fistful Of Vengeance (2022).jpg".to_string(),
        mediaid:"873020358496385403".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Fistful Of Vengeance (2022).mp4".to_string(),
        movname:"FistfulOfVengeance".to_string(),
        movyear:"2022".to_string(),
        thumbpath:"./static/Fistful Of Vengeance (2022).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Gladiator (2000).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Gladiator (2000).jpg".to_string(),
        mediaid:"385735028708390798".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Gladiator (2000).mp4".to_string(),
        movname:"Gladiator".to_string(),
        movyear:"2000".to_string(),
        thumbpath:"./static/Gladiator (2000).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Hard Target (1993).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Hard Target (1993).jpg".to_string(),
        mediaid:"1581572479152063605".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Hard Target (1993).mp4".to_string(),
        movname:"HardTarget".to_string(),
        movyear:"1993".to_string(),
        thumbpath:"./static/Hard Target (1993).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Insurgent (2015).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Insurgent (2015).jpg".to_string(),
        mediaid:"1204418104937116117".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Insurgent (2015).mp4".to_string(),
        movname:"Insurgent".to_string(),
        movyear:"2015".to_string(),
        thumbpath:"./static/Insurgent (2015).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Lara Croft Tomb Raider (2001).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Lara Croft Tomb Raider (2001).jpg".to_string(),
        mediaid:"950849937548559643".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Lara Croft Tomb Raider (2001).mp4".to_string(),
        movname:"LaraCroftTombRaider".to_string(),
        movyear:"2001".to_string(),
        thumbpath:"./static/Lara Croft Tomb Raider (2001).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Lara Croft Tomb Raider The Craddle Of Life (2003).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Lara Croft Tomb Raider The Craddle Of Life (2003).jpg".to_string(),
        mediaid:"27337836140904823".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Lara Croft Tomb Raider The Craddle Of Life (2003).mp4".to_string(),
        movname:"LaraCroftTombRaiderTheCraddleOfLife".to_string(),
        movyear:"2003".to_string(),
        thumbpath:"./static/Lara Croft Tomb Raider The Craddle Of Life (2003).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Life Of Pi (2012).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Life Of Pi (2012).jpg".to_string(),
        mediaid:"1425943107991092770".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Life Of Pi (2012).mp4".to_string(),
        movname:"LifeOfPi".to_string(),
        movyear:"2012".to_string(),
        thumbpath:"./static/Life Of Pi (2012).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Mad Max Fury Road (2015).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Mad Max Fury Road (2015).jpg".to_string(),
        mediaid:"1380557323745039047".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Mad Max Fury Road (2015).mp4".to_string(),
        movname:"MadMaxFuryRoad".to_string(),
        movyear:"2015".to_string(),
        thumbpath:"./static/Mad Max Fury Road (2015).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Man Down (2015).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Man Down (2015).jpg".to_string(),
        mediaid:"1472496983578913874".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Man Down (2015).mp4".to_string(),
        movname:"ManDown".to_string(),
        movyear:"2015".to_string(),
        thumbpath:"./static/Man Down (2015).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Men Of Honor (2000).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Men Of Honor (2000).jpg".to_string(),
        mediaid:"1104662326553994636".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Men Of Honor (2000).mp4".to_string(),
        movname:"MenOfHonor".to_string(),
        movyear:"2000".to_string(),
        thumbpath:"./static/Men Of Honor (2000).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Mulan (2020).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Mulan (2020).jpg".to_string(),
        mediaid:"746893478768580272".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Mulan (2020).mp4".to_string(),
        movname:"Mulan".to_string(),
        movyear:"2020".to_string(),
        thumbpath:"./static/Mulan (2020).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Safe House (2012).mkv".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Safe House (2012).jpg".to_string(),
        mediaid:"577970987180957870".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Safe House (2012).mkv".to_string(),
        movname:"SafeHouse".to_string(),
        movyear:"2012".to_string(),
        thumbpath:"./static/Safe House (2012).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Sahara (2005).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Sahara (2005).jpg".to_string(),
        mediaid:"216039380063098388".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Sahara (2005).mp4".to_string(),
        movname:"Sahara".to_string(),
        movyear:"2005".to_string(),
        thumbpath:"./static/Sahara (2005).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Secret Headquarters (2022).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Secret Headquarters (2022).jpg".to_string(),
        mediaid:"1214149993609986134".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Secret Headquarters (2022).mp4".to_string(),
        movname:"SecretHeadquarters".to_string(),
        movyear:"2022".to_string(),
        thumbpath:"./static/Secret Headquarters (2022).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Sweet Girl (2021).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Sweet Girl (2021).jpg".to_string(),
        mediaid:"532327207612288963".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Sweet Girl (2021).mp4".to_string(),
        movname:"SweetGirl".to_string(),
        movyear:"2021".to_string(),
        thumbpath:"./static/Sweet Girl (2021).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/The Commuter (2018).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/The Commuter (2018).jpg".to_string(),
        mediaid:"1400564906993645562".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/The Commuter (2018).mp4".to_string(),
        movname:"TheCommuter".to_string(),
        movyear:"2018".to_string(),
        thumbpath:"./static/The Commuter (2018).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/The Expendables (2010).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/The Expendables (2010).jpg".to_string(),
        mediaid:"1418203167730481706".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/The Expendables (2010).mp4".to_string(),
        movname:"TheExpendables".to_string(),
        movyear:"2010".to_string(),
        thumbpath:"./static/The Expendables (2010).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/The Expendables 2 (2012).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/The Expendables 2 (2012).jpg".to_string(),
        mediaid:"28764386887952075".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/The Expendables 2 (2012).mp4".to_string(),
        movname:"TheExpendables2".to_string(),
        movyear:"2012".to_string(),
        thumbpath:"./static/The Expendables 2 (2012).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/The Expendables 3 (2014).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/The Expendables 3 (2014).jpg".to_string(),
        mediaid:"163382345510524712".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/The Expendables 3 (2014).mp4".to_string(),
        movname:"TheExpendables3".to_string(),
        movyear:"2014".to_string(),
        thumbpath:"./static/The Expendables 3 (2014).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/The Fate Of The Furious (2017).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/The Fate Of The Furious (2017).jpg".to_string(),
        mediaid:"1222909087948857419".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/The Fate Of The Furious (2017).mp4".to_string(),
        movname:"TheFateOfTheFurious".to_string(),
        movyear:"2017".to_string(),
        thumbpath:"./static/The Fate Of The Furious (2017).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/The Gray Man (2022).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/The Gray Man (2022).jpg".to_string(),
        mediaid:"831347796112783956".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/The Gray Man (2022).mp4".to_string(),
        movname:"TheGrayMan".to_string(),
        movyear:"2022".to_string(),
        thumbpath:"./static/The Gray Man (2022).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/The Lost City (2022).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/The Lost City (2022).jpg".to_string(),
        mediaid:"347258830523953365".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/The Lost City (2022).mp4".to_string(),
        movname:"TheLostCity".to_string(),
        movyear:"2022".to_string(),
        thumbpath:"./static/The Lost City (2022).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/The Northman (2022).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/The Northman (2022).jpg".to_string(),
        mediaid:"772576529551810477".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/The Northman (2022).mp4".to_string(),
        movname:"TheNorthman".to_string(),
        movyear:"2022".to_string(),
        thumbpath:"./static/The Northman (2022).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/ThreeHundred (2006).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/ThreeHundred (2006).jpg".to_string(),
        mediaid:"1033337124160690621".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/ThreeHundred (2006).mp4".to_string(),
        movname:"ThreeHundred".to_string(),
        movyear:"2006".to_string(),
        thumbpath:"./static/ThreeHundred (2006).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Tomb Raider (2018).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Tomb Raider (2018).jpg".to_string(),
        mediaid:"951914639056492956".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Tomb Raider (2018).mp4".to_string(),
        movname:"TombRaider".to_string(),
        movyear:"2018".to_string(),
        thumbpath:"./static/Tomb Raider (2018).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Twelve Strong (2018).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Twelve Strong (2018).jpg".to_string(),
        mediaid:"1514788103988546529".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Twelve Strong (2018).mp4".to_string(),
        movname:"TwelveStrong".to_string(),
        movyear:"2018".to_string(),
        thumbpath:"./static/Twelve Strong (2018).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/TwentyTwelve (2009).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/TwentyTwelve (2009).jpg".to_string(),
        mediaid:"1537115404555238342".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/TwentyTwelve (2009).mp4".to_string(),
        movname:"TwentyTwelve".to_string(),
        movyear:"2009".to_string(),
        thumbpath:"./static/TwentyTwelve (2009).jpg".to_string()
    },

    Video {
        catagory:"Action".to_string(),
        dirpath:"/root/fsData/Movies/Action/".to_string(),
        filepath:"/root/fsData/Movies/Action/Unstoppable (2010).mp4".to_string(),
        genre:"movies".to_string(),
        httpthumbpath:"http://192.168.0.94:8888/static/Unstoppable (2010).jpg".to_string(),
        mediaid:"1190657437215426681".to_string(), 
        movfspath:"/media/pi/PiTB/media/Movies/Action/Unstoppable (2010).mp4".to_string(),
        movname:"Unstoppable".to_string(),
        movyear:"2010".to_string(),
        thumbpath:"./static/Unstoppable (2010).jpg".to_string()
    }
];
        
      
    let img_div = use_style!("
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: center;
        align-items: center;
        margin-bottom: 60px;
    ");
    let img_style = use_style!("
        display: block;
        margin-top: 10px;
        margin-right: auto;
        margin-left: auto;
        width: 200px;
        border-radius: 8px;
    ");

    

    let videos = videos.iter().map(|video| html! {
            <>
            
                <img 
                    id={video.mediaid.clone()}
                    key={video.mediaid.clone()} 
                    class={img_style.clone()} 
                    src={video.httpthumbpath.clone()} 
                    alt="video thumbnail" 
                />
            
            </>
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
pub fn ActionPage() -> Html {

    let btn_group = use_style!(
        "
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: space-evenly;
        align-items: center;
        margin-top: 1em;
        margin-bottom: 1em;
    "
    );

    let h1_style = use_style!("
        text-align: center;
        color: #ebb917;
    ");

    
    html!(
        <>
            <div class={btn_group}>
                <crate::comps::mainpage_comps::MusicSVG />
                <crate::comps::mainpage_comps::MovieSVG />
                <crate::comps::mainpage_comps::TVShowsSVG />
            </div>
            <h1 class={h1_style}>{"Action"}</h1>
            <crate::comps::mainpage_comps::PlayerControls />
            <ImageGroupThree />
            <crate::comps::mainpage_comps::PlayerControls />
        </>
    )
}