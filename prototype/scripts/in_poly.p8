pico-8 cartridge // http://www.pico-8.com
version 32
__lua__
function _init()
	poke(0x5f2d,1)
	cls(5)
	m={x=stat(32),y=stat(33)}
	d=false
	p={}
	s=0
	n=6
	// b={}
	// n=12
	// for y=0,n do for x=0,n do
	//		add(b,{
	//		 x=(64+x*128)/n,
	//		 y=(64+y*128)/n,
	//	 	w=0
	// 	})
	// end end
	c={1,8,9,12,10}
end

function _update60()
	cls(5)
	m={x=stat(32),y=stat(33)}
	
	if stat(34)==1 then
		d=true
	elseif d then
		d=false
		add(p,m)
	end
	
	if #p>2 then
		add(p,p[1])
		s=wn(m,p)
		for y=n/2,127+n/2,n do for x=n/2,127+n/2,n do
			rectfill(x-n/2,y-n/2,x+n/2,y+n/2,c[wn({x=x,y=y},p)])
		end end
		deli(p,#p)
	end
end

// ![](http://web.archive.org/web/20210504233957/http://geomalgorithms.com/a03-_inclusion.html)
// Copyright 2001, 2012, 2021 Dan Sunday
// This code may be freely used and modified for any purpose
// providing that this copyright notice is included with it.
// There is no warranty for this code, and the author of it cannot
// be held liable for any real or imagined damage from its use.
// Users of this code must verify correctness for their application.

function is_left(p0,p1,p2)
	return (p1.x-p0.x)*(p2.y-p0.y)
							-(p2.x-p0.x)*(p1.y-p0.y)
end

function wn(m,v)
	local wn=0

 for i=1,#v-1 do
  if v[i].y<=m.y then
  	if v[i+1].y>m.y then
  		if is_left(v[i],v[i+1],m)>0 then
   		wn+=1
   	end
  	end
  elseif v[i+1].y<=m.y then
   if is_left(v[i],v[i+1],m)<0 then
   	wn-=1
   end
  end
 end

 return wn
end

function _draw()
	//cls(5)
	
//	for k in all(b) do
//		rectfill(k.x-64/n,k.y-64/n,k.x+64/n,k.y+64/n,c[k.w+1])
//	end
//	
//	for k in all(b) do
//		print(k.w,k.x-1,k.y-2,6)
//	end
	
	for v in all(p) do
		circfill(v.x,v.y,2,11)
		if(o) line(o.x,o.y,v.x,v.y,7)
		o=v
	end

	spr(1,m.x,m.y)
	rectfill(m.x+6,m.y,m.x+14,m.y+6,0)
	print(s,m.x+7,m.y+1,8)
end
__gfx__
00000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000171000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00700700177100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00077000177710000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00077000177771000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00700700177110000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000011710000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
