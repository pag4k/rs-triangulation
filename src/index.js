/***
 * Excerpted from "Programming WebAssembly with Rust",
 * published by The Pragmatic Bookshelf.
 * Copyrights apply to this code. It may not be used to create training material,
 * courses, books, articles, and the like. Contact us if you are in doubt.
 * We make no guarantees that this code is fit for any purpose.
 * Visit http://www.pragmaticprogrammer.com/titles/khrust for more book information.
***/
function generate() {
  fetch('../target/wasm32-unknown-unknown/release/rs_triangulation.wasm').then(response =>
    response.arrayBuffer()
  ).then(bytes => WebAssembly.instantiate(bytes)).then(results => {
    console.log("Starting application...");
    instance = results.instance;

    var regex = /[+-]?\d+(?:\.\d+)?/g;

    //var numbers = string.match(regex).map(Number);

    numberString = document.getElementById('numbers').value;

    var match;
    var coordinates = [];
    while (match = regex.exec(numberString)) {
      coordinates.push(match[0]);
      if (coordinates.length == 2) {
        instance.exports.add_vertex(coordinates[0], coordinates[1]);
        coordinates = [];
      }
    }

    const width = 1600;
    const height = 800;

    console.log("Add vertices...");
    const number_vertices = 25000;
    // for (let i = 0; i < number_vertices; ++i) {
    //   let x = Math.random() * width;
    //   let y = Math.random() * height;
    //   //console.log("Vertex: (" + x + "," + y + ").");
    //   instance.exports.add_vertex(x, y);
    // }

    console.log("Triangulate...");
    let number_eges = instance.exports.triangulate();


    var canvas = document.getElementById('Canvas');
    //Always check for properties and methods, to make sure your code doesn't break in other browsers.
    if (canvas.getContext) {
      var context = canvas.getContext('2d');

      for (let i = 0; i < number_eges; i++) {
        let x1 = instance.exports.get_x1_at(i);
        let x2 = instance.exports.get_x2_at(i);
        let y1 = instance.exports.get_y1_at(i);
        let y2 = instance.exports.get_y2_at(i);
        //console.log("Edge: (" + x1 + "," + y1 + ") -> (" + x2 + "," + y2 + ").");

        // Reset the current path
        context.beginPath();
        // Staring point (10,45)
        context.moveTo(x1, y1);
        // End point (180,47)
        context.lineTo(x2, y2);
        // Make the line visible
        context.stroke();
      }


    }

    document.getElementById("container").innerText = number_eges;
  }).catch(console.error);
}