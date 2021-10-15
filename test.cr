
@[Link(ldflags: "#{__DIR__}/libhi.so")]
lib LibHi
  struct Conf
    name : LibC::Char*
    age : Int32
  end


  fun test(_msg : LibC::Char*): Conf
end

conf = LibHi.test("{name: 'Jake', age: 32}")

puts String.new(conf.name)