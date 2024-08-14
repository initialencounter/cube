use crate::types::Face;

pub fn rot(face: Face, rev: bool) -> Face {
    let mut result_face = [[0; 3]; 3];
    if rev {
        let mut i_ = 0;
        for i in 0..3 {
            let mut j_ = 0;
            for j in (0..3).rev() {
                result_face[j_][i_] = face[i][j];
                j_ += 1;
            }
            i_ += 1;
        }
    } else {
        let mut i_ = 0;
        for i in (0..3).rev() {
            let mut j_ = 0;
            for j in 0..3 {
                result_face[j_][i_] = face[i][j];
                j_ += 1;
            }
            i_ += 1;
        }
    }
    result_face
}

pub fn get_color(id: i8) -> &'static str {
    match id {
        1..=9 => "blue",
        10..=18 => "green",
        19..=27 => "orange",
        28..=36 => "red",
        37..=45 => "yellow",
        46..=54 => "white",
        _ => "transparent",
    }
}

pub fn make_svg(cube: [[[i8; 3]; 3]; 9]) -> String {
    let svg_template = [r"<svg id='cube' data-name='cube' xmlns='http://www.w3.org/2000/svg'>
      <!-- L orange -->
      <rect class='1' x='0.25' y='44.96' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M12.39,49.21v12H.5v-12H12.39m.5-.5H0v13H12.89v-13Z' transform='translate(0 -4)'/>
      <rect class='2' x='14.43' y='44.96' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M26.57,49.21v12H14.68v-12H26.57m.5-.5H14.18v13H27.07v-13Z' transform='translate(0 -4)'/>
      <rect class='3' x='28.61' y='44.96' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M40.75,49.21v12H28.86v-12H40.75m.5-.5H28.36v13H41.25v-13Z' transform='translate(0 -4)'/>
      <rect class='4' x='0.25' y='59.25' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M12.39,63.5v12H.5v-12H12.39m.5-.5H0V76H12.89V63Z' transform='translate(0 -4)'/>
      <rect class='5' x='14.43' y='59.25' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M26.57,63.5v12H14.68v-12H26.57m.5-.5H14.18V76H27.07V63Z' transform='translate(0 -4)'/>
      <rect class='6' x='28.61' y='59.25' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M40.75,63.5v12H28.86v-12H40.75m.5-.5H28.36V76H41.25V63Z' transform='translate(0 -4)'/>
      <rect class='7' x='0.25' y='73.54' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M12.39,77.79v12H.5v-12H12.39m.5-.5H0v13H12.89v-13Z' transform='translate(0 -4)'/>
      <rect class='8' x='14.43' y='73.54' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M26.57,77.79v12H14.68v-12H26.57m.5-.5H14.18v13H27.07v-13Z' transform='translate(0 -4)'/>
      <rect class='9' x='28.61' y='73.54' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M40.75,77.79v12H28.86v-12H40.75m.5-.5H28.36v13H41.25v-13Z' transform='translate(0 -4)'/>

      <!-- F blue -->
      <rect class='1' x='44.58' y='44.96' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M56.72,49.21v12H44.83v-12H56.72m.5-.5H44.33v13H57.22v-13Z' transform='translate(0 -4)'/>
      <rect class='2' x='58.76' y='44.96' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M70.9,49.21v12H59v-12H70.9m.5-.5H58.51v13H71.4v-13Z' transform='translate(0 -4)'/>
      <rect class='3' x='72.94' y='44.96' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M85.08,49.21v12H73.19v-12H85.08m.5-.5H72.69v13H85.58v-13Z' transform='translate(0 -4)'/>
      <rect class='4' x='44.58' y='59.25' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M56.72,63.5v12H44.83v-12H56.72m.5-.5H44.33V76H57.22V63Z' transform='translate(0 -4)'/>
      <rect class='5' x='58.76' y='59.25' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M70.9,63.5v12H59v-12H70.9m.5-.5H58.51V76H71.4V63Z' transform='translate(0 -4)'/>
      <rect class='6' x='72.94' y='59.25' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M85.08,63.5v12H73.19v-12H85.08m.5-.5H72.69V76H85.58V63Z' transform='translate(0 -4)'/>
      <rect class='7' x='44.58' y='73.54' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M56.72,77.79v12H44.83v-12H56.72m.5-.5H44.33v13H57.22v-13Z' transform='translate(0 -4)'/>
      <rect class='8' x='58.76' y='73.54' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M70.9,77.79v12H59v-12H70.9m.5-.5H58.51v13H71.4v-13Z' transform='translate(0 -4)'/>
      <rect class='9' x='72.94' y='73.54' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M85.08,77.79v12H73.19v-12H85.08m.5-.5H72.69v13H85.58v-13Z' transform='translate(0 -4)'/>

      <!-- U yellow -->
      <rect class='1' x='44.58' y='0.25' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M56.72,4.5v12H44.83V4.5H56.72m.5-.5H44.33V17H57.22V4Z' transform='translate(0 -4)'/>
      <rect class='2' x='58.76' y='0.25' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M70.9,4.5v12H59V4.5H70.9m.5-.5H58.51V17H71.4V4Z' transform='translate(0 -4)'/>
      <rect class='3' x='72.94' y='0.25' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M85.08,4.5v12H73.19V4.5H85.08m.5-.5H72.69V17H85.58V4Z' transform='translate(0 -4)'/>
      <rect class='4' x='44.58' y='14.54' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M56.72,18.79v12H44.83v-12H56.72m.5-.5H44.33v13H57.22v-13Z' transform='translate(0 -4)'/>
      <rect class='5' x='58.76' y='14.54' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M70.9,18.79v12H59v-12H70.9m.5-.5H58.51v13H71.4v-13Z' transform='translate(0 -4)'/>
      <rect class='6' x='72.94' y='14.54' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M85.08,18.79v12H73.19v-12H85.08m.5-.5H72.69v13H85.58v-13Z' transform='translate(0 -4)'/>
      <rect class='7' x='44.58' y='28.83' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M56.72,33.08v12H44.83v-12H56.72m.5-.5H44.33v13H57.22v-13Z' transform='translate(0 -4)'/>
      <rect class='8' x='58.76' y='28.83' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M70.9,33.08v12H59v-12H70.9m.5-.5H58.51v13H71.4v-13Z' transform='translate(0 -4)'/>
      <rect class='9' x='72.94' y='28.83' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M85.08,33.08v12H73.19v-12H85.08m.5-.5H72.69v13H85.58v-13Z' transform='translate(0 -4)'/>

      <!-- D white -->
      <rect class='1' x='44.58' y='89.68' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M56.72,93.93v12H44.83v-12H56.72m.5-.5H44.33v13H57.22v-13Z' transform='translate(0 -4)'/>
      <rect class='2' x='58.76' y='89.68' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M70.9,93.93v12H59v-12H70.9m.5-.5H58.51v13H71.4v-13Z' transform='translate(0 -4)'/>
      <rect class='3' x='72.94' y='89.68' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M85.08,93.93v12H73.19v-12H85.08m.5-.5H72.69v13H85.58v-13Z' transform='translate(0 -4)'/>
      <rect class='4' x='44.58' y='103.97' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M56.72,108.22v12H44.83v-12H56.72m.5-.5H44.33v13H57.22v-13Z' transform='translate(0 -4)'/>
      <rect class='5' x='58.76' y='103.97' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M70.9,108.22v12H59v-12H70.9m.5-.5H58.51v13H71.4v-13Z' transform='translate(0 -4)'/>
      <rect class='6' x='72.94' y='103.97' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M85.08,108.22v12H73.19v-12H85.08m.5-.5H72.69v13H85.58v-13Z' transform='translate(0 -4)'/>
      <rect class='7' x='44.58' y='118.26' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M56.72,122.51v12H44.83v-12H56.72m.5-.5H44.33v13H57.22V122Z' transform='translate(0 -4)'/>
      <rect class='8' x='58.76' y='118.26' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M70.9,122.51v12H59v-12H70.9m.5-.5H58.51v13H71.4V122Z' transform='translate(0 -4)'/>
      <rect class='9' x='72.94' y='118.26' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M85.08,122.51v12H73.19v-12H85.08m.5-.5H72.69v13H85.58V122Z' transform='translate(0 -4)'/>

      <!-- R red -->
      <rect class='1' x='89.82' y='44.96' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M102,49.21v12H90.07v-12H102m.5-.5H89.57v13h12.89v-13Z' transform='translate(0 -4)'/>
      <rect class='2' x='104' y='44.96' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M116.14,49.21v12H104.25v-12h11.89m.5-.5H103.75v13h12.89v-13Z' transform='translate(0 -4)'/>
      <rect class='3' x='118.18' y='44.96' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M130.32,49.21v12H118.43v-12h11.89m.5-.5H117.93v13h12.89v-13Z' transform='translate(0 -4)'/>
      <rect class='4' x='89.82' y='59.25' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M102,63.5v12H90.07v-12H102m.5-.5H89.57V76h12.89V63Z' transform='translate(0 -4)'/>
      <rect class='5' x='104' y='59.25' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M116.14,63.5v12H104.25v-12h11.89m.5-.5H103.75V76h12.89V63Z' transform='translate(0 -4)'/>
      <rect class='6' x='118.18' y='59.25' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M130.32,63.5v12H118.43v-12h11.89m.5-.5H117.93V76h12.89V63Z' transform='translate(0 -4)'/>
      <rect class='7' x='89.82' y='73.54' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M102,77.79v12H90.07v-12H102m.5-.5H89.57v13h12.89v-13Z' transform='translate(0 -4)'/>
      <rect class='8' x='104' y='73.54' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M116.14,77.79v12H104.25v-12h11.89m.5-.5H103.75v13h12.89v-13Z' transform='translate(0 -4)'/>
      <rect class='9' x='118.18' y='73.54' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M130.32,77.79v12H118.43v-12h11.89m.5-.5H117.93v13h12.89v-13Z' transform='translate(0 -4)'/>

      <!-- B green -->
      <rect class='1' x='134.15' y='44.96' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M146.29,49.21v12H134.4v-12h11.89m.5-.5H133.9v13h12.89v-13Z' transform='translate(0 -4)'/>
      <rect class='2' x='148.33' y='44.96' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M160.47,49.21v12H148.58v-12h11.89m.5-.5H148.08v13H161v-13Z' transform='translate(0 -4)'/>
      <rect class='3' x='162.51' y='44.96' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M174.65,49.21v12H162.76v-12h11.89m.5-.5H162.26v13h12.89v-13Z' transform='translate(0 -4)'/>
      <rect class='4' x='134.15' y='59.25' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M146.29,63.5v12H134.4v-12h11.89m.5-.5H133.9V76h12.89V63Z' transform='translate(0 -4)'/>
      <rect class='5' x='148.33' y='59.25' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M160.47,63.5v12H148.58v-12h11.89m.5-.5H148.08V76H161V63Z' transform='translate(0 -4)'/>
      <rect class='6' x='162.51' y='59.25' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M174.65,63.5v12H162.76v-12h11.89m.5-.5H162.26V76h12.89V63Z' transform='translate(0 -4)'/>
      <rect class='7' x='134.15' y='73.54' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M146.29,77.79v12H134.4v-12h11.89m.5-.5H133.9v13h12.89v-13Z' transform='translate(0 -4)'/>
      <rect class='8' x='148.33' y='73.54' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M160.47,77.79v12H148.58v-12h11.89m.5-.5H148.08v13H161v-13Z' transform='translate(0 -4)'/>
      <rect class='9' x='162.51' y='73.54' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M174.65,77.79v12H162.76v-12h11.89m.5-.5H162.26v13h12.89v-13Z' transform='translate(0 -4)'/>

      <!-- F blue -->
      <rect class='1' x='178.15' y='43.96' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M190.29,48.21v12H178.4v-12h11.89m.5-.5H177.9v13h12.89v-13Z' transform='translate(0 -4)'/>
      <rect class='2' x='192.33' y='43.96' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M204.47,48.21v12H192.58v-12h11.89m.5-.5H192.08v13H205v-13Z' transform='translate(0 -4)'/>
      <rect class='3' x='206.51' y='43.96' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M218.65,48.21v12H206.76v-12h11.89m.5-.5H206.26v13h12.89v-13Z' transform='translate(0 -4)'/>
      <rect class='4' x='178.15' y='58.25' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M190.29,62.5v12H178.4v-12h11.89m.5-.5H177.9V75h12.89V62Z' transform='translate(0 -4)'/>
      <rect class='5' x='192.33' y='58.25' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M204.47,62.5v12H192.58v-12h11.89m.5-.5H192.08V75H205V62Z' transform='translate(0 -4)'/>
      <rect class='6' x='206.51' y='58.25' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M218.65,62.5v12H206.76v-12h11.89m.5-.5H206.26V75h12.89V62Z' transform='translate(0 -4)'/>
      <rect class='7' x='178.15' y='72.54' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M190.29,76.79v12H178.4v-12h11.89m.5-.5H177.9v13h12.89v-13Z' transform='translate(0 -4)'/>
      <rect class='8' x='192.33' y='72.54' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M204.47,76.79v12H192.58v-12h11.89m.5-.5H192.08v13H205v-13Z' transform='translate(0 -4)'/>
      <rect class='9' x='206.51' y='72.54' width='12.39' height='12.49' fill='",
        r"'/>
      <path d='M218.65,76.79v12H206.76v-12h11.89m.5-.5H206.26v13h12.89v-13Z' transform='translate(0 -4)'/>

      <!-- U yellow -->
      <polygon class='1' points='195.33 24.3 202.25 16.68 214.34 16.68 207.43 24.3 195.33 24.3' fill='",
        r"'/>
      <path d='M202.36,20.93h11.42l-6.47,7.12H195.9l6.46-7.12m-.22-.5-7.37,8.12h12.77l7.37-8.12Z' transform='translate(0 -4)'/>
      <polygon class='2' points='209.38 24.3 216.29 16.68 228.39 16.68 221.47 24.3 209.38 24.3' fill='",
        r"'/>
      <path d='M227.82,20.93l-6.46,7.12H209.94l6.47-7.12h11.41m1.13-.5H216.18l-7.37,8.12h12.77L229,20.43Z' transform='translate(0 -4)'/>
      <polygon class='3' points='223.42 24.3 230.34 16.68 242.44 16.68 235.52 24.3 223.42 24.3' fill='",
        r"'/>
      <path d='M241.87,20.93l-6.46,7.12H224l6.46-7.12h11.42m1.13-.5H230.23l-7.37,8.12h12.77L243,20.43Z' transform='translate(0 -4)'/>
      <polygon class='4' points='187.22 33.23 194.14 25.62 206.23 25.62 199.31 33.23 187.22 33.23' fill='",
        r"'/>
      <path d='M205.67,29.87,199.2,37H187.79l6.46-7.11h11.42m1.13-.5H194l-7.37,8.11h12.77l7.37-8.11Z' transform='translate(0 -4)'/>
      <polygon class='5' points='201.27 33.23 208.19 25.62 220.28 25.62 213.36 33.23 201.27 33.23' fill='",
        r"'/>
      <path d='M219.71,29.87,213.25,37H201.83l6.47-7.11h11.41m1.13-.5H208.07l-7.37,8.11h12.77l7.37-8.11Z' transform='translate(0 -4)'/>
      <polygon class='6' points='215.31 33.23 222.23 25.62 234.33 25.62 227.41 33.23 215.31 33.23' fill='",
        r"'/>
      <path d='M222.34,29.87h11.42L227.3,37H215.88l6.46-7.11m-.22-.5-7.37,8.11h12.77l7.37-8.11Z' transform='translate(0 -4)'/>
      <polygon class='7' points='179.11 42.17 186.03 34.55 198.12 34.55 191.21 42.17 179.11 42.17' fill='",
        r"'/>
      <path d='M197.56,38.8l-6.47,7.12H179.68l6.46-7.12h11.42m1.13-.5H185.92l-7.37,8.12h12.77l7.37-8.12Z' transform='translate(0 -4)'/>
      <polygon class='8' points='193.16 42.17 200.08 34.55 212.17 34.55 205.25 42.17 193.16 42.17' fill='",
        r"'/>
      <path d='M211.61,38.8l-6.47,7.12H193.72l6.47-7.12h11.42m1.12-.5H200l-7.38,8.12h12.77l7.37-8.12Z' transform='translate(0 -4)'/>
      <polygon class='9' points='207.2 42.17 214.12 34.55 226.22 34.55 219.3 42.17 207.2 42.17' fill='",
        r"'/>
      <path d='M225.65,38.8l-6.46,7.12H207.77l6.46-7.12h11.42m1.13-.5H214l-7.37,8.12h12.77l7.37-8.12Z' transform='translate(0 -4)'/>

      <!-- R red -->
      <polygon class='1' points='220.69 44.04 227.24 36.24 227.24 48.38 220.69 56.18 220.69 44.04' fill='",
        r"'/>
      <path d='M227,40.93V52.29l-6.05,7.21V48.13l6.05-7.2m.5-1.37L220.44,48V60.87l7.05-8.4V39.56Z' transform='translate(0 -4)'/>
      <polygon class='2' points='228.45 34.81 235 27.01 235 39.15 228.45 46.95 228.45 34.81' fill='",
        r"'/>
      <path d='M234.75,31.69V43.05l-6.05,7.21V38.9l6.05-7.21m.5-1.37-7.05,8.4V51.63l7.05-8.39V30.32Z' transform='translate(0 -4)'/>
      <polygon class='3' points='236.2 25.57 242.75 17.77 242.75 29.91 236.2 37.71 236.2 25.57' fill='",
        r"'/>
      <path d='M242.5,22.46V33.82L236.45,41V29.66l6.05-7.2m.5-1.38L236,29.48V42.4L243,34V21.08Z' transform='translate(0 -4)'/>
      <polygon class='4' points='220.69 58.25 227.24 50.45 227.24 62.59 220.69 70.39 220.69 58.25' fill='",
        r"'/>
      <path d='M227,55.14V66.5l-6.05,7.2V62.34l6.05-7.2m.5-1.38-7.05,8.4V75.08l7.05-8.4V53.76Z' transform='translate(0 -4)'/>
      <polygon class='5' points='228.45 49.02 235 41.22 235 53.35 228.45 61.15 228.45 49.02' fill='",
        r"'/>
      <path d='M234.75,45.9V57.26l-6.05,7.21V53.11l6.05-7.21m.5-1.37-7.05,8.39V65.84l7.05-8.4V44.53Z' transform='translate(0 -4)'/>
      <polygon class='6' points='236.2 39.78 242.75 31.98 242.75 44.12 236.2 51.92 236.2 39.78' fill='",
        r"'/>
      <path d='M242.5,36.67V48l-6.05,7.2V43.87l6.05-7.2m.5-1.38L236,43.69V56.61l7.05-8.4V35.29Z' transform='translate(0 -4)'/>
      <polygon class='7' points='220.69 72.46 227.24 64.66 227.24 76.8 220.69 84.6 220.69 72.46' fill='",
        r"'/>
      <path d='M227,69.35V80.71l-6.05,7.2V76.55l6.05-7.2m.5-1.38-7.05,8.4V89.29l7.05-8.4V68Z' transform='translate(0 -4)'/>
      <polygon class='8' points='228.45 63.22 235 55.42 235 67.56 228.45 75.36 228.45 63.22' fill='",
        r"'/>
      <path d='M234.75,60.11V71.47l-6.05,7.21V67.31l6.05-7.2m.5-1.37-7.05,8.39V80.05l7.05-8.4V58.74Z' transform='translate(0 -4)'/>
      <polygon class='9' points='236.2 53.99 242.75 46.19 242.75 58.33 236.2 66.13 236.2 53.99' fill='",
        r"'/>
      <path d='M242.5,50.87V62.24l-6.05,7.2V58.08l6.05-7.21m.5-1.37L236,57.9V70.81L243,62.42V49.5Z' transform='translate(0 -4)'/>
</svg>"];
    let mut svg = String::from("");
    let cube: Vec<_>  = cube.iter()
        .flat_map(|matrix| matrix.iter())
        .flat_map(|row| row.iter())
        .cloned()
        .collect();
    for i in 0..81 {
        svg.push_str(format!("{}{}", svg_template[i], get_color(cube[i])).as_str());
    }
    svg.push_str(svg_template[81]);
    svg
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let face = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ];
        let result = rot(face, false);
        assert_eq!(result, [
            [7, 4, 1],
            [8, 5, 2],
            [9, 6, 3]
        ]);
    }

    #[test]
    fn it_works_rev() {
        let face = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ];
        let result = rot(face, true);
        assert_eq!(result, [
            [3, 6, 9],
            [2, 5, 8],
            [1, 4, 7]
        ]);
    }
}