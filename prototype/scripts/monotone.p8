pico-8 cartridge // http://www.pico-8.com
version 32
__lua__
function _init()
	bs={}
	for i=1,4 do
		local h
		if rnd(1)>.5 then
			h=1
		else
			h=-1
		end
		
		local v
		if rnd(1)>.5 then
			v=1
		else
			v=-1
		end
		
		add(bs,{
			p={
				x=32+rnd(64),
				y=32+rnd(64),
			},
			r=32+rnd(16),
			h=h,
			v=v,
		})
	end
end

function _update()
	
end

function _draw()
	cls(5)
	
	is=find_ints(bs)
	
	for b in all(bs) do
		local o
		local d=6
		for n=0,d do
			local v=point(b,n/d)
			if(o)line(o.x,o.y,v.x,v.y,11)
			o=v
		end
		
		pset(b.p.x,b.p.y,9)
		pset(b.p.x+b.r*b.h,b.p.y+b.r*b.v,9)
		pset(b.p.x,b.p.y+b.r*b.v,9)
	end
	
	for s in all(is) do
		local vi=point(bs[s.i],s.s.i)
		local vj=point(bs[s.j],s.s.j)
		circfill(vj.x,vj.y,2,12)
		circfill(vi.x,vi.y,1,8)
	end
end

function find_ints(bs)
	local is={}
	
	for i,b1 in pairs(bs) do
		for j,b2 in pairs(bs) do
			if i~=j then
				for s in all(
					ints(b1,b2,1,1,0,0)
				) do
					add(is,{i=i,j=j,s=s})
				end
			end
		end
	end
	
	return is
end
-->8
function point(b,t)
	return {
		x=bezier(
			t,
			b.p.x,
			b.p.x,
			b.p.x+b.r*b.h),
		y=bezier(
			t,
			b.p.y,
			b.p.y+b.r*b.v,
			b.p.y+b.r*b.v),
	}
end

function lerp(t,a,b)
	return a+t*(b-a)
end

function bezier(t,a,b,c)
	return lerp(t,lerp(t,a,b),lerp(t,b,c))
end
-->8
function ints(b1,b2,o1,o2,t1,t2)
	local i=point(b1,t1)
	local a=point(b1,t1+o1)
	local a1={
		i={
			x=min(i.x,a.x),
			y=min(i.y,a.y)
		},
		a={
			x=max(i.x,a.x),
			y=max(i.y,a.y)
		},
	}
	
	local i=point(b2,t2)
	local a=point(b2,t2+o2)
	local a2={
		i={
			x=min(i.x,a.x),
			y=min(i.y,a.y)
		},
		a={
			x=max(i.x,a.x),
			y=max(i.y,a.y)
		},
	}
	
	--pset(a1.i.x,a1.i.y,11)
	--pset(a1.a.x,a1.a.y,11)
	
	--pset(a2.i.x,a2.i.y,11)
	--pset(a2.a.x,a2.a.y,11)
	
	if aabb(a1,a2) then
		--rect(a1.i.x,a1.i.y,a1.a.x,a1.a.y,6)
		--rect(a2.i.x,a2.i.y,a2.a.x,a2.a.y,6)
	
		if o1<0.005 then
			return {{i=t1+o1/2,j=t2+o2/2}}
		else
			local is={}
			
			for s in all(
				ints(
					b1,b2,
					o1/2,o2/2,
					t1,t2
				)) do
				add(is,s)
			end
			
			for s in all(
				ints(
					b1,b2,
					o1/2,o2/2,
					t1+o1/2,t2
				)) do
				add(is,s)
			end
			
			for s in all(
				ints(
					b1,b2,
					o1/2,o2/2,
					t1,t2+o2/2
				)) do
				add(is,s)
			end
			
			for s in all(
				ints(
					b1,b2,
					o1/2,o2/2,
					t1+o1/2,t2+o2/2
				)) do
				add(is,s)
			end
			
			return is
		end
	else
		return {}
	end
end

function aabb(a1,a2)
	return 
	a1.i.x< a2.a.x and
	a1.a.x>=a2.i.x and
	a1.i.y< a2.a.y and
	a1.a.y>=a2.i.y
end
__gfx__
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00700700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00077000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00077000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00700700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
