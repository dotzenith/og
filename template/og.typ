#set page(
  width: 600pt,
  height: 315pt,
  margin: 0pt,
  background: image(sys.inputs.at("bg", default: "bg.png"), width: 100%, height: 100%)
)

#set text(font: "Berkeley Mono", fill: rgb(30, 30, 36))

#place(
  horizon, 
  dx: 240pt, 
  block(
    width: 60%,
    text(
      size: int(sys.inputs.at("size", default: "30")) * 1pt, 
      weight: "semibold", 
      sys.inputs.at("title", default: "OG")
    )
  )
)

#place(
  bottom + right,
  dy: -5pt,
  dx: -5pt,
  block(
    text(
      size: 15pt, 
      weight: "semibold", 
      sys.inputs.at("date", default: "OG")
    )
  )
)
