package org.nativescript.node_compat.demo

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import org.nativescript.node_compat.buffer.Buffer
import org.nativescript.node_compat.buffer.decodeHex
import org.nativescript.node_compat.demo.ui.theme.NodeCompatDemoTheme
import org.nativescript.node_compat.fs.FileStat
import org.nativescript.node_compat.fs.FileSystem
import java.math.BigInteger
import java.nio.ByteBuffer

class MainActivity : ComponentActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)
    setContent {
      NodeCompatDemoTheme {
        // A surface container using the 'background' color from the theme
        Surface(modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background) {
          Greeting("Android")
        }
      }
    }

    val encoded = Buffer.btoa("Osei Fortune")
    println(encoded)
    val decoded = Buffer.atob(encoded)
    println(decoded)

    var buffer = Buffer.from("hello world", Buffer.Encoding.Utf8)

    println(buffer.length())

    var buf = buffer.buffer()

    println(buf)

    Log.d("Buffer", buffer.toString(Buffer.Encoding.Hex))
// Prints: 68656c6c6f20776f726c64
    println(buffer.toString(Buffer.Encoding.Base64))
// Prints: aGVsbG8gd29ybGQ=


    println(Buffer.from("fhqwhgads", Buffer.Encoding.Utf8))
// Prints: <Buffer 66 68 71 77 68 67 61 64 73>
    println(Buffer.from("fhqwhgads", Buffer.Encoding.Utf16le));
// Prints: <Buffer 66 00 68 00 71 00 77 00 68 00 67 00 61 00 64 00 73 00>


    val buffer1 = ByteBuffer.allocateDirect(3)
    buffer1.put(byteArrayOf(1, 2, 3), 0, 3)
    buffer1.rewind()

    val buffer2 = ByteBuffer.allocateDirect(3)
    buffer2.put(byteArrayOf(4, 5, 6), 0, 3)
    buffer2.rewind()

    val buffer3 = ByteBuffer.allocateDirect(3)
    buffer3.put(byteArrayOf(7, 8, 9), 0, 3)
    buffer3.rewind()

    //  buffer = Buffer.concat(arrayOf(buffer1, buffer2, buffer3))

    // println(buffer);


    buffer = Buffer.alloc(11, "aGVsbG8gd29ybGQ=", Buffer.Encoding.Base64)

    println(buffer)

    buffer = Buffer.alloc(26)

    buf = buffer.buffer()

    buf?.let { buf ->
      for (i in 0 until 26) {
        buf.put(i, ((i + 97).toByte()))
      }
    }

    println(buffer.toString(Buffer.Encoding.Utf8))
// Prints: abcdefghijklmnopqrstuvwxyz
    println(buffer.toString(Buffer.Encoding.Utf8, 0, 5))
// Prints: abcde


    buffer = Buffer.from(byteArrayOf(0x12, 0x34, 0x56, 0x78))

    println(buffer.readUInt32LE(0));
// Prints: 78563412
    // println!("{:x}",buffer.read_uint32le(Some(1)).unwrap());
// Throws ERR_OUT_OF_RANGE.


    buffer = Buffer.alloc(8)

    val a = 0x0102030405060708

    buffer.writeBigInt64BE(a.toBigInteger(), 0)

    println(buffer);


    buffer = Buffer.alloc(8)

    val value = 0x0102030405060708
    buffer.writeBigInt64LE(value.toBigInteger(), 0)

    println(buffer)


    buffer = Buffer.alloc(8)

    buffer.writeBigUInt64BE(BigInteger("0xdecafafecacefade".decodeHex()), 0)

    println(buffer)


    buffer = Buffer.alloc(8)

    buffer.writeBigUInt64LE(BigInteger("0xdecafafecacefade".decodeHex()), 0);

    println(buffer)


    buffer = Buffer.alloc(4)

    buffer.writeUInt8(0x3, 0)
    buffer.writeUInt8(0x4, 1)
    buffer.writeUInt8(0x23, 2)
    buffer.writeUInt8(0x42, 3)

    println(buffer)
// Prints: <Buffer 03 04 23 42>


    buffer = Buffer.alloc(2)

    buffer.writeInt16BE(0x0102, 0);

    println(buffer);
// Prints: <Buffer 01 02>


    buffer = Buffer.alloc(4)

    buffer.writeUInt16BE(0xdead.toShort(), 0)
    buffer.writeUInt16BE(0xbeef.toShort(), 2)

    println(buffer)
// Prints: <Buffer de ad be ef>


  }
}

@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {
  Text(
    text = "Hello $name!",
    modifier = modifier
  )
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
  NodeCompatDemoTheme {
    Greeting("Android")
  }
}
