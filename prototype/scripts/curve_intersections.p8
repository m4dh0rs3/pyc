pico-8 cartridge // http://www.pico-8.com
version 32
__lua__
function _init()
	poke(0x5f2d,1)

	cs={}
	for i=1,6 do
		add(cs,{
			x=16+rnd(96),
			y=16+rnd(96),
			r=8+rnd(8),
			s=rnd(1),
			o=(rnd(2)-1),
		})
	end
	
	d=12
end

function _update()
	cs[1].x=stat(32)
	cs[1].y=stat(33)
	
	is={}
	for i,s in pairs(cs) do
		for j,o in pairs(cs) do
			if i~=j then
				for p in all(cc(s,o)) do
					add(is,p)
				end
			end
		end
	end
end

function _draw()
	cls(5)
	
	for c in all(cs) do
		circ(c.x,c.y,c.r,13)
		local o
		for n=1,d do
			local a=c.r+c.o*n/d
			local v={
				x=c.x+c.r*cos(a),
				y=c.y+c.r*sin(a),
			}
			if(o) line(o.x,o.y,v.x,v.y,7)
			o=v
		end
	end
	
	for p in all(is) do
		circfill(p.x,p.y,2,p.c)
		circfill(
			p.s.x+p.s.r*cos(p.as),
			p.s.y+p.s.r*sin(p.as),
			1,14)
		circfill(
			p.o.x+p.o.r*cos(p.ao),
			p.o.y+p.o.r*sin(p.ao),
			1,10)
	end
end

function in_c(c,a)
	local e=nm(c.s+c.o)

	if c.o>0 then
		if c.s>e then
			return not (a>e and a<c.s)
		else
			return a>=c.s and a<=e
		end
	else
		if c.s<e then
			return not (a>c.s and a<e)
		else
			return a>=e and a<=c.s
		end
	end
end

function nm(a)
	if a>0 then
		return a-flr(a)
	else
		return a+ceil(a)
	end
end

function cc(s,o)
	// offsets between circle centers
	local off={
		x=o.x-s.x,
		y=o.y-s.y,
	}
	
	// straight line distance between the centers
 local dist=sqrt(off.x^2+off.y^2)

	// check for solvability, circles should neither be contained, nor distant
	if dist<=s.r+o.r and dist>=abs(s.r-o.r) then
		
		// distance from point 0 to point 2
		local tan=(s.r^2-o.r^2+dist^2)/(2*dist)

		local md={
			x=s.x+(off.x*(tan/dist)),
			y=s.y+(off.y*(tan/dist)),
		}
		
		// distance from point 2 to either of the intersection points
		local hgt=sqrt(s.r^2-tan^2)
		
		// offsets of the intersection points from point 2
		local hyp={
			x=-off.y*(hgt/dist),
			y= off.x*(hgt/dist),
		}
		
		local ps={}
		if hyp.x==0 and hyp.y==0 then
			add(ps,md)
		else
			add(ps,{x=md.x+hyp.x,y=md.y+hyp.y})
			add(ps,{x=md.x-hyp.x,y=md.y-hyp.y})
		end
	
		for p in all(ps) do
			p.s=s
			p.o=o
			
			p.as=ag({
				x=p.x-s.x,
				y=p.y-s.y
			})

			p.ao=ag({
				x=p.x-o.x,
				y=p.y-o.y
			})
			
			if in_c(s,p.as) and in_c(o,p.ao) then
				p.c=11
			else
				p.c=8
			end
		end
		
		return ps
	else
		return {}
	end
end

function ag(p)
	return atan2(p.x,p.y)
end
__gfx__
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00700700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00077000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00077000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00700700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
