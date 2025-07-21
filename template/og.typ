#let data = json(bytes(sys.inputs.data))

#set page(
  width: 600pt,
  height: 315pt,
  margin: 0pt,
  background: image(data.at("background", default: "background.png"), width: 100%, height: 100%)
)

#set text(font: "Berkeley Mono", fill: rgb(30, 30, 36))

// The Main Text
#place(
  horizon, 
  dx: 240pt, 
  block(
    width: 60%,
    text(
      size: int(data.at("size", default: "30")) * 1pt, 
      weight: "semibold", 
      data.at("title", default: "Can't even have OpenGraph images around here")
    )
  )
)

// Blog Name
#place(
  top + right,
  dy: 5pt,
  dx: -5pt,
  block(
    text(
      size: 15pt, 
      weight: "semibold",
      data.at("website", default: "blog.danshu.co")
    )
  )
)

// Date
#place(
  bottom + right,
  dy: -5pt,
  dx: -5pt,
  block(
    text(
      size: 15pt, 
      weight: "semibold", 
      data.at("date", default: "04 Apr 2025")
    )
  )
)
