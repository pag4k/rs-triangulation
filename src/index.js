/***
 * Excerpted from "Programming WebAssembly with Rust",
 * published by The Pragmatic Bookshelf.
 * Copyrights apply to this code. It may not be used to create training material,
 * courses, books, articles, and the like. Contact us if you are in doubt.
 * We make no guarantees that this code is fit for any purpose.
 * Visit http://www.pragmaticprogrammer.com/titles/khrust for more book information.
***/

const url = 'rs_triangulation.wasm';
// var importObject = { imports: { imported_func: arg => console.log(arg) } };



function generate() {
  fetch(url).then(response =>
    response.arrayBuffer()
  ).then(bytes => WebAssembly.instantiate(bytes,
    {
      env: {
        alert: function (ptr, number) {
          let str = copyCStr(Module, ptr);
          alert(str + " -> " + number);
        }
        //     notify_progress: (percentage) => {
        //       console.log(100 * percentage + "%");
        //       document.getElementById("progress").innerText = 100 * percentage + "%";
        //     }
      }
    }

  )).then(results => results.instance)
    .then(instance => {
      const Module = {
        alloc: instance.exports.alloc,
        dealloc: instance.exports.dealloc,
        triangulate: instance.exports.triangulate,
        memory: instance.exports.memory
      };

      console.log("Starting application...");
      //        instance = results.instance;

      var NumberArray = [];
      ReadInput(NumberArray);

      console.log("Read " + NumberArray.length + " numbers...");

      var InputPointer = WriteBuffer(Module, NumberArray);

      console.log("Wrote to buffer at  " + InputPointer + "...");

      var OutputPointer = Module.triangulate(InputPointer, NumberArray.length);

      console.log("Triangulated with output at " + OutputPointer + "...");

      DrawEdges(Module, OutputPointer);

      Module.dealloc(InputPointer, 4 * NumberArray);

      //const width = 1600;
      //const height = 800;

      //console.log("Added " + vertices + " vertices...");
      //const number_vertices = 25000;
      // for (let i = 0; i < number_vertices; ++i) {
      //   let x = Math.random() * width;
      //   let y = Math.random() * height;
      //   //console.log("Vertex: (" + x + "," + y + ").");
      //   instance.exports.add_vertex(x, y);
      // }

      // console.log("Triangulate...");
      // let number_eges = instance.exports.triangulate();

      // console.log("Drawing " + number_eges + " edges");

      // var canvas = document.getElementById('Canvas');
      // //Always check for properties and methods, to make sure your code doesn't break in other browsers.
      // if (canvas.getContext) {
      //   var context = canvas.getContext('2d');
      //   context.clearRect(0, 0, canvas.width, canvas.height);

      //   for (let i = 0; i < number_eges; i++) {
      //     let x1 = instance.exports.get_x1_at(i);
      //     let x2 = instance.exports.get_x2_at(i);
      //     let y1 = instance.exports.get_y1_at(i);
      //     let y2 = instance.exports.get_y2_at(i);
      //     //console.log("Edge: (" + x1 + "," + y1 + ") -> (" + x2 + "," + y2 + ").");

      //     // Reset the current path
      //     context.beginPath();
      //     // Staring point (10,45)
      //     context.moveTo(x1, y1);
      //     // End point (180,47)
      //     context.lineTo(x2, y2);
      //     // Make the line visible
      //     context.stroke();
      //   }

      // }


    });
}

function toBytesInt32(num) {
  arr = new Uint8Array([
    (num & 0xff000000) >> 24,
    (num & 0x00ff0000) >> 16,
    (num & 0x0000ff00) >> 8,
    (num & 0x000000ff)
  ]);
  return arr;
}

function u8s_to_u32(o, p, q, r) {
  return (o << 24) | (p << 16) | (q << 8) | r;
}

function ReadInput(NumberArray) {
  var regex = /[+-]?\d+(?:\.\d+)?/g;

  var numberString = document.getElementById('numbers').value;

  var match;
  while (match = regex.exec(numberString)) {
    NumberArray.push(match[0]);
  }
  // If the number of numbers is odd, remove last.
  if (NumberArray.length % 2 == 1) {
    NumberArray.pop();
  }
}

function WriteBuffer(Module, NumberArray) {
  const BufferLength = 4 * NumberArray.length;
  const pointer = Module.alloc(BufferLength);
  const memory = new Uint8Array(Module.memory.buffer);
  for (let i = 0; i < NumberArray.length; ++i) {
    var bytes = toBytesInt32(NumberArray[i]);
    for (let j = 0; j < bytes.length; ++j) {
      memory[pointer + 4 * i + j] = bytes[j]
    }
  }
  return pointer;
}

function DrawEdges(Module, Pointer) {
  const Buffer = new Uint8Array(Module.memory.buffer.slice(Pointer));
  const NumberOfEdges = ReadInt32(Buffer, 0); //u8s_to_u32(Buffer[0], Buffer[1], Buffer[2], Buffer[3]);

  console.log("Drawing  " + NumberOfEdges + " edges...");

  if (0 >= NumberOfEdges) {
    return;
  }

  let Offset = 4;

  var canvas = document.getElementById('Canvas');
  //Always check for properties and methods, to make sure your code doesn't break in other browsers.
  if (canvas.getContext) {
    var context = canvas.getContext('2d');
    context.clearRect(0, 0, canvas.width, canvas.height);

    for (let i = 0; i < NumberOfEdges; i++) {
      let x1 = ReadInt32(Buffer, Offset + 16 * i + 0)
      let y1 = ReadInt32(Buffer, Offset + 16 * i + 4)
      let x2 = ReadInt32(Buffer, Offset + 16 * i + 8)
      let y2 = ReadInt32(Buffer, Offset + 16 * i + 12)
      console.log("Edge: (" + x1 + "," + y1 + ") -> (" + x2 + "," + y2 + ").");
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

  Module.dealloc(Pointer, Pointer + Offset + 16 * NumberOfEdges);
}

function ReadInt32(Buffer, Pointer) {
  return u8s_to_u32(Buffer[Pointer + 0], Buffer[Pointer + 1], Buffer[Pointer + 2], Buffer[Pointer + 3]);
}