
const url = './src/rs_triangulation.wasm';

function generate() {
  fetch(url)
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes))
    .then(results => results.instance)
    .then(instance => {
      const Module = {
        alloc: instance.exports.alloc,
        dealloc: instance.exports.dealloc,
        triangulate: instance.exports.triangulate,
        memory: instance.exports.memory
      };

      console.log("Starting application...");

      var NumberArray = [];
      ReadInput(NumberArray);

      console.log("Read " + NumberArray.length + " numbers...");

      var InputPointer = WriteBuffer(Module, NumberArray);

      console.log("Wrote to buffer at  " + InputPointer + "...");

      var OutputPointer = Module.triangulate(InputPointer, NumberArray.length);

      console.log("Triangulated with output at " + OutputPointer + "...");

      DrawEdges(Module, OutputPointer);

      Module.dealloc(InputPointer, 4 * NumberArray);
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
  const NumberOfEdges = ReadInt32(Buffer, 0);

  console.log("Drawing  " + NumberOfEdges + " edges...");

  if (0 >= NumberOfEdges) {
    return;
  }

  let Offset = 4;

  var canvas = document.getElementById('Canvas');
  if (canvas.getContext) {
    var context = canvas.getContext('2d');
    context.clearRect(0, 0, canvas.width, canvas.height);

    for (let i = 0; i < NumberOfEdges; i++) {
      let x1 = ReadInt32(Buffer, Offset + 16 * i + 0)
      let y1 = ReadInt32(Buffer, Offset + 16 * i + 4)
      let x2 = ReadInt32(Buffer, Offset + 16 * i + 8)
      let y2 = ReadInt32(Buffer, Offset + 16 * i + 12)
      context.beginPath();
      context.moveTo(x1, y1);
      context.lineTo(x2, y2);
      context.stroke();
    }

  }

  Module.dealloc(Pointer, Pointer + Offset + 16 * NumberOfEdges);
}

function ReadInt32(Buffer, Pointer) {
  return u8s_to_u32(Buffer[Pointer + 0], Buffer[Pointer + 1], Buffer[Pointer + 2], Buffer[Pointer + 3]);
}

function get_random_int() {
  const Number = document.getElementById('randomText').value
  const Max = 1000;
  let integers = [];
  for (let i = 0; i < Number; ++i) {
    integers.push(Math.trunc(Math.random() * Max));
  }
  document.getElementById('numbers').value = integers.join(' ');
}