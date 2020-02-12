
let icons = ./sketch/icons.dhall

let composition =
        λ(x : Integer)
      → λ(y : Integer)
      → λ(size : Natural)
      → { placement =
            { absolute = { x = x, y = y } }
        , size =
            size
        , query =
            { icon = "head1" }
        }

in  { width =
        600
    , height =
        600
    , compositions =
        [ composition +100 +100 50
        , composition +200 +100 50
        , composition +300 +100 50
        ]
    , icons =
        icons
    }
