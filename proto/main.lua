function printf(k, i)
    if type(k) == "table" then
        for i, v in ipairs(k) do
            printf(v, i)
        end
    elseif not i then
        print(k)
    else
        print(i..": "..k)
    end
end

function add_edge(u, v)
    if not graph[u] then
        graph[u]={}
    end

    if not graph[v] then
        graph[v]={}
    end

    table.insert(graph[u], v)
    table.insert(graph[v], u)
end

function dfs_cycle(u, p, color, mark, par)
    if color[u] == 2 then
        return
    end

    if color[u] == 1 then
        cyclenumber = cyclenumber + 1
        local cur = p
        mark[cur] = cyclenumber

        while cur ~= u do
            cur = par[cur] or 0
            mark[cur] = cyclenumber
        end

        return
    end

    par[u] = p

    color[u] = 1

    for k, v in ipairs(graph[u] or {}) do
        if v ~= (par[u] or 0) then
            dfs_cycle(v, u, color, mark, par)
        end
    end

    color[u] = 2
end

function print_cycles(num_edges, mark)
    for i = 1, num_edges do
        if (mark[i] or 0) ~= 0 then
            if not cycles[mark[i]] then
                cycles[mark[i]] = {}
            end

            table.insert(cycles[mark[i]], i)
        end
    end

    for i = 1, cyclenumber do
        local node_poly = {}
        for k, x in ipairs(cycles[i] or {}) do
            table.insert(node_poly, x)
        end

        if #node_poly > 1 then
            table.insert(node_polys, node_poly)
            debug_node_poly(node_poly)
        end
    end
end

function debug_node_poly(node_poly)
    str = "Poly: "
    for k, x in ipairs(node_poly) do
        str = str..k..", "
    end
    print(str)
    print()
end

function find_polys()
    node_polys = {}

    cyclenumber = 0
    num_edges = #edges

    graph = {}
    cycles = {}
    color = {}
    par = {}
    mark = {}

    for i, edge in ipairs(edges) do
        if edge ~= nil then
            add_edge(edge.i, edge.j)
        end
    end

    --for i = 0, #edges - 1 do
    dfs_cycle(1, 0, color, mark, par)
    --end

    print_cycles(num_edges, mark)

    polys = {}

    for i, node_poly in ipairs(node_polys) do
        local poly_path = {}
        local verts = {}

        for j = 1, #node_poly do
            local node_i = node_poly[j]
            local node_j = node_poly[j + 1]
            if j == #node_poly then node_j = node_poly[1] end

            for k, edge in ipairs(edges) do
                if edge.i == node_i and
                   edge.j == node_j then
                    table.insert(verts, edge.s)
                    table.insert(verts, edge.e)

                    for l = 1, #edge.path do
                        table.insert(poly_path, edge.path[l])
                    end
                elseif edge.i == node_j and
                       edge.j == node_i then
                    table.insert(verts, edge.s)
                    table.insert(verts, edge.e)

                    for l = #edge.path, 1, - 1 do
                        table.insert(poly_path, edge.path[l])
                    end
                end
            end
        end

        if #poly_path > 0 then
            table.insert(polys, {path = poly_path, verts = verts})
        end
    end
end

function common_value(array)
    val = {}
	
	for i, v in ipairs(array) do
		local hit = false
		
		for j, k in ipairs(val) do
			if k.v == v then
				k.p = k.p + 1
				hit = true
				break
			end
		end
		
		if not hit then
			table.insert(val, {v = v, p = 1})
		end
	end
	
	m = 0
	
	for i, v in ipairs(val) do
		if v.p > m then
			l = v.v
			m = v.p
		end
	end
	
	return l
end

function near(a, b)
    return math.abs(round(a) - round(b)) <= 2
end

function point_in_poly(x, y, poly)
    local on = false
    kols={}
        
    for angle = 0, (math.pi*2) - (math.pi/3), math.pi/3 do
        local kol = 0
            
        local o = false
        for m, v in ipairs(poly.path) do
            if o then
                local intersection = line_intersection(
                    {x = x, y = y},
                    {x = x + (height * math.cos(angle)), y = y + (height * math.sin(angle))},
                    v, o
                )
                            
                if intersection ~= false then
                    kol = kol + 1
                end

                if (near(v.x, x) and near(v.y, y)) or (near(o.x, x) and near(o.y, y)) then
                    on = true
                end
            end

            o = v
        end

        table.insert(kols, kol)
    end

    for m, v in ipairs(poly.verts) do
        if (near(v.x, x) and near(v.y, y)) then
            on = true
        end
    end
                    
    kol = common_value(kols)
                            
    if ((kol + 2) % 2 == 1) or on then
        return true
    else
        return false
    end
end

function update_points()
    for v = 1, grid.resolution do
        for u = 1, grid.resolution do
            if grid.points[v][u] == 0 then
                for i, poly in ipairs(polys) do
                    local x, y = n2x(u, v)
                    if point_in_poly(x, y, poly) then
                        grid.points[v][u] = arrow.current
                    end
                end
            end
        end
    end
end

function draw_poly(poly)
    o = false
    for i, v in ipairs(poly.path) do
        if o then
            love.graphics.line(
                v.x, v.y, o.x, o.y
            )
        end

        o = v
    end
end

function draw_polys()
    if #polys ~= 0 then
        love.graphics.setColor(scheme.node)

        for i, poly in ipairs(polys) do
            draw_poly(poly)
        end
    end
end

-- ** GRID ** --
function new_grid(x, y, width, height, items_width, items_height, space, draw)
    local grid = {
        x = x, y = y,
        width = width,
        height = height,
        items_width = items_width,
        items_height = items_height,
        item_width = (width - ((items_width - 1) * space)) / items_width,
        item_height = (height - ((items_height - 1) * space)) / items_height,
        space = space,
        draw = draw or true
    }

    return grid
end

function item_dimensions_in_grid(grid, i, j)
    local dimensions = {
        x = round(grid.x + ((i - 1) * grid.space) + ((i - 1) * grid.item_width)),
        y = round(grid.y + ((j - 1) * grid.space) + ((j - 1) * grid.item_height)),
        width = round(grid.item_width),
        height = round(grid.item_height)
    }

    return dimensions
end

function draw_grid(grid)
    love.graphics.setColor(scheme.background)
    love.graphics.rectangle("fill", button_grid.x - 10, button_grid.y - 10, button_grid.width + 10, button_grid.height + 10)

    love.graphics.setLineWidth(2)
    love.graphics.setColor(scheme.white)
    
    for i = 2, grid.items_width do
        local grid_x = grid.x + ((i - 1) * grid.space) + ((i - 1) * grid.item_width) - (grid.space / 2)

        love.graphics.line(grid_x, grid.y, grid_x, grid.y + grid.height)
    end

    for j = 2, grid.items_height do
        local grid_y = grid.y + ((j - 1) * grid.space) + ((j - 1) * grid.item_height) - (grid.space / 2)

        love.graphics.line(grid.x, grid_y, grid.x + grid.width, grid_y)
    end
end

-- ** BUTTONS ** --

function new_button(call, image, x, y, width, height, flip)
    local button = {
        image = image,
        x = x, y = y,
        over = false,
        width = width,
        height = height,
        call = call,
        flip = flip,
        pressed = false
    }

    button.image_width, button.image_height = button.image:getDimensions()
    button.scale_x = button.width / button.image_width
    button.scale_y = button.height / button.image_height

    if flip then
        button.scale_x = -1 * button.scale_x
    end

    table.insert(buttons, button)
end

function in_button(mx, my, button)
    if mx >= button.x and mx <= button.x + button.width and
       my >= button.y and my <= button.y + button.height then
        return true
    else
        return false
    end
end

function draw_button(button)
    love.graphics.setColor(scheme.white)
    
    if button.over and not button.pressed then
        if button.flip then
            love.graphics.draw(
                button.image, 
                button.x + button.width + 2, button.y + 2,
                0, button.scale_x, button.scale_y
            )
        else
            love.graphics.draw(
                button.image, 
                button.x + 2, button.y + 2,
                0, button.scale_x, button.scale_y
            )
        end
    else
        if button.flip then
            love.graphics.draw(
                button.image, 
                button.x + button.width, button.y,
                0, button.scale_x, button.scale_y
            )
        else
            love.graphics.draw(
                button.image, 
                button.x, button.y,
                0, button.scale_x, button.scale_y
            )
        end
    end
end

function draw_buttons()
    for i, button in ipairs(buttons) do
        draw_button(button)
    end
end

function draw_points()
    for v = 1, grid.resolution do
        for u = 1, grid.resolution do
            local x, y = n2x(u, v)
            
            if grid.points[v][u] == 1 then
                love.graphics.setColor(scheme.one)
            elseif grid.points[v][u] == 2 then
                love.graphics.setColor(scheme.two)
            else
                love.graphics.setColor(scheme.white)
            end

            love.graphics.circle("fill", x, y, 3)
        end
    end
end

function count_points()
    player.one.points = 0
    player.two.points = 0
    
    for v = 1, grid.resolution do
        for u = 1, grid.resolution do
            if grid.points[v][u] == 1 then
                player.one.points = player.one.points + 1
            elseif grid.points[v][u] == 2 then
                player.two.points = player.two.points + 1
            end
        end
    end
end

function n2x(u, v)
    return u * grid.scale + grid.offset.x, v * grid.scale + grid.offset.y
end

function x2n(x, y)
    return round((x - grid.offset.x) / grid.scale), round((y - grid.offset.y) / grid.scale)
end

function draw_counter()
    love.graphics.setColor(scheme.white)
    
    love.graphics.draw(images.counter, 0, 0)

    love.graphics.setFont(frostbite)

    love.graphics.setColor(scheme.one)
    love.graphics.printf(
        player.one.points, 
        width / 6 + counter.height / 4, 
        counter.height / 2, 95, 
        "center")
    
    love.graphics.arc(
        "fill", 
        width / 6, counter.height / 2, 
        counter.height / 4, 
        0, remap(player.one.time_left, 0, counter.time, 0.1,  2 * math.pi - 0.1))

    love.graphics.setColor(scheme.two)
    love.graphics.printf(
        player.two.points, 
        width * 5/6 - counter.height / 4 - 95, 
        counter.height / 2, 95, 
        "center")

    love.graphics.arc(
        "fill", 
        width * 5/6, counter.height / 2, 
        counter.height / 4, 
        math.pi, math.pi - remap(player.two.time_left, 0.1, counter.time, 0,  2 * math.pi - 0.1))

    love.graphics.setColor(scheme.white)

    love.graphics.circle(
        "line", 
        width / 6, counter.height / 2, 
        counter.height / 4)

    love.graphics.arc(
        "line", 
        width / 6, counter.height / 2, 
        counter.height / 4, 
        0, remap(player.one.time_left, 0, counter.time, 0.1,  2 * math.pi - 0.1))

    love.graphics.circle(
        "line", 
        width * 5/6, counter.height / 2, 
        counter.height / 4)

    love.graphics.arc(
        "line", 
        width * 5/6, counter.height / 2, 
        counter.height / 4, 
        math.pi, math.pi - remap(player.two.time_left, 0, counter.time, 0.1,  2 * math.pi - 0.1))
end

function draw_arrow()
    arrow.x, arrow.y = n2x(arrow.u, arrow.v)

    love.graphics.draw(
        images.arrow, 
        arrow.x, arrow.y, 
        arrow.a, 
        arrow.scale_x, arrow.scale_y, 
        arrow.image_width/2, arrow.image_height/2
    )
    
    love.graphics.origin()
end

function find_arrow_node()
    for l, node in ipairs(nodes) do
        if near(arrow.x, node.x) and near(arrow.y, node.y) then
            return l
        end
    end
end

function round(v)
    return math.floor(v + 0.5)
end

function lerpPQ(p, q, t)
    return {
        x = p.x + t * (q.x - p.x),
        y = p.y + t * (q.y - p.y)
    }
end

function new_bezier(p, q, c, d)
    local m = {}

    for t = 0, 1, 1/d do
        table.insert(m,
            lerpPQ(lerpPQ(p, c, t), lerpPQ(c, q, t), t)
    )
    end

    return m
end

function new_edge(i, j, path, owner, c, step)
    local edge = {
        i = i,
        j = j,

        s = nodes[i],
        e = nodes[j],
        c = c,

        path = path,
        line = {},

        owner = owner,
        step = step
    }

    remove_doubles(edge.path)

    for i, v in ipairs(edge.path) do
        table.insert(edge.line, v.x)
        table.insert(edge.line, v.y)
    end

    table.insert(edges, edge)
    
    return #edges
end

function remove_doubles(path)
    for i = 1, #path do
        local v = path[i]

        if i>1 and v~=nil then
            local o = path[i-1]

            if near(v.x, o.x) and near(v.y, o.y) then
                table.remove(path, i)
            end
        end
    end
end

function cut_edge(edge, edge_i, k, m, l, s)
    if l and s then
        if l < k then
            k, l = l, k
            m, s = s, m
        end

        local path1 = {}--edge.s}
        local path2 = {m}
        local path3 = {s}

        for i, v in ipairs(edge.path) do
            if i < k then
                table.insert(path1, v)
            elseif i < l then
                table.insert(path2, v)
            else
                table.insert(path3, v)
            end
        end

        table.insert(path1, m)
        table.insert(path2, s)
        --table.insert(path3, edge.e)

        local i = new_node(m.x, m.y)
        local j = new_node(s.x, s.y)

        table.remove(edges, edge_i)

        local a = new_edge(edge.i, i, path1, edge.owner, edge.c, edge.step)
        local b = new_edge(i, j, path2, edge.owner, edge.c, edge.step)
        local c = new_edge(j, edge.j, path3, edge.owner, edge.c, edge.step)
        arc_intersects(edges[a], a)
        arc_intersects(edges[b], b)
        arc_intersects(edges[c], c)
    else
        local path1 = {}--edge.s}
        local path2 = {m}

        for i, v in ipairs(edge.path) do
            if i < k then
                table.insert(path1, v)
            else
                table.insert(path2, v)
            end
        end

        table.insert(path1, m)
        --table.insert(path2, edge.e)

        local i = new_node(m.x, m.y)


        local a = new_edge(edge.i, i, path1, edge.owner, edge.c, edge.step)
        local b = new_edge(i, edge.j, path2, edge.owner, edge.c, edge.step)
        arc_intersects(edges[a], a)
        arc_intersects(edges[b], b)


        table.remove(edges, edge_i)
    end
end

function remap(x, a, b, c, d)
    return x / (b - a) * (d - c) + c
end

function has_intersectors(edge, i)
    for j, secd in ipairs(edges) do
        if i ~= j then
            local o = false
            for k, v in ipairs(edge.path) do
                if o then  
                    local d = false
                    for m, u in ipairs(secd.path) do
                        if d then
                            if line_intersection(v, o, u, d) ~= false then
                                return true
                            end
                        end
                    end
                end

                o = v
            end
        end
    end

    return false
end

function draw_edge(edge)
    love.graphics.line(edge.line)
    
    --love.graphics.line(edge.s.x, edge.s.y, edge.e.x, edge.e.y)
    --love.graphics.print(edge.i.."-"..edge.j, edge.c.x, edge.c.y)
end

function draw_edges()
    love.graphics.setColor(scheme.white)
    love.graphics.setLineWidth(2)
    love.graphics.setFont(mono)
    
    for i, edge in ipairs(edges) do
        draw_edge(edge)
    end
end

function new_node(x, y)
    for i, node in ipairs(nodes) do
        if near(node.x, x) and near(node.y, y) then
            return i
        end
    end

    local node = {
        x = x,
        y = y,
        i = #nodes + 1
    }

    table.insert(nodes, node)

    return #nodes
end

function draw_node(node)
    love.graphics.circle("fill", node.x, node.y, 4)
    love.graphics.print(node.i, node.x, node.y)
end

function draw_nodes()
    love.graphics.setColor(scheme.node)
    love.graphics.setFont(mono)

    for i, node in ipairs(nodes) do
        draw_node(node)
    end
end

function a2n(a)
    return (a - (math.floor(a/(2*math.pi))*2*math.pi))
end

function line_intersection(p1, p2, p3, p4)
    local s1= {
        x = p2.x - p1.x,
        y = p2.y - p1.y
    }
    
    local s2= {
        x = p4.x - p3.x,
        y = p4.y - p3.y
    }
   
    s = (-s1.y * (p1.x - p3.x) + s1.x * (p1.y - p3.y)) / (-s2.x * s1.y + s1.x * s2.y)
    t = ( s2.x * (p1.y - p3.y) - s2.y * (p1.x - p3.x)) / (-s2.x * s1.y + s1.x * s2.y)
       
    if s >= 0 and s <= 1 and t >= 0 and t <= 1 then
        return {
            x = p1.x + (t * s1.x),
            y = p1.y + (t * s1.y),
            s = s, t = t
        }
    end
   
    return false
end


function arc_intersects(edge, i)
    for j, secd in ipairs(edges) do
        local intersections = {}
        
        if math.abs(i - j) > 1 and edge.step ~= secd.step then
            local o = false
            
            for k, v in ipairs(edge.path) do
                if o then
                    local d = false

                    for l, u in ipairs(secd.path) do
                        if d then
                            local intersection = line_intersection(v, o, u, d)

                            if intersection then
                                if (math.abs(i - j) ~= 1) or
                                   (math.abs(i - j) == 1 and
                                    intersection.t >= 0.08 and
                                    intersection.t <= 0.92 and
                                    intersection.s >= 0.08 and
                                    intersection.s <= 0.92) and
                                    ((k ~= 1 and l ~= 1) or 
                                    (k ~= #edge.path and l ~= #secd.path)) then

                                    brk = false

                                    for m, node in ipairs(nodes) do
                                        if near(intersection.x, node.x) and
                                           near(intersection.y, node.y) then
                                            brk = true
                                        end
                                    end

                                    if not brk then
                                        table.insert(intersections, {
                                            i = i, j = j,
                                            k = k, l = l,
                                            v = v, o = o,
                                            u = u, d = d,
                                            t = intersection.t,
                                            s = intersection.s,
                                            x = intersection.x,
                                            y = intersection.y,
                                            secd = secd
                                        })
                                    end
                                end
                            end
                        end

                        d = u
                    end
                end

                o = v
            end
        end

        if #intersections == 1 then
            cut_edge(
                edge, i, 
                intersections[1].k,
                {x = intersections[1].x, y = intersections[1].y}
            )

            cut_edge(
                intersections[1].secd, intersections[1].j, 
                intersections[1].l,
                {x = intersections[1].x, y = intersections[1].y}
            )

        elseif #intersections == 2 then
            cut_edge(
                edge, i, 
                intersections[1].k, 
                {x = intersections[1].x, y = intersections[1].y}, 
                intersections[2].k, 
                {x = intersections[2].x, y = intersections[2].y}
            )

            cut_edge(
                intersections[1].secd, intersections[1].j, 
                intersections[1].l, 
                {x = intersections[1].x, y = intersections[1].y}, 
                intersections[2].l, 
                {x = intersections[2].x, y = intersections[2].y}
            )
        end
    end
end

function next_arc(r, a)
    a = math.pi * (((2 * a) - 1) / 4)
    a = a2n(a)
    arrow.a = a2n(arrow.a)

    arrow.x, arrow.y = n2x(arrow.u, arrow.v)

    local e = {
        x = arrow.x + (r * grid.quad_scale) * math.cos(arrow.a + a - math.pi/2), 
        y = arrow.y + (r * grid.quad_scale) * math.sin(arrow.a + a - math.pi/2)
    }

    e.x, e.y = n2x(x2n(e.x, e.y))

    local c = nil

    if a2n(a + math.pi/2) < math.pi then
        c = {
            x = arrow.x + (r * grid.scale) * math.cos(arrow.a - math.pi/2),
            y = arrow.y + (r * grid.scale) * math.sin(arrow.a - math.pi/2)
        }
    else
        c = {
            x = arrow.x + (r * grid.scale) * math.cos(arrow.a + math.pi - math.pi/2),
            y = arrow.y + (r * grid.scale) * math.sin(arrow.a + math.pi - math.pi/2)
        }
    end

    c.x, c.y = n2x(x2n(c.x, c.y))

    local back = false

    for i, secd in ipairs(edges) do
        if c.x == secd.c.x and c.y == secd.c.y and
           ((secd.s.x == arrow.x and secd.s.y == arrow.y) or
            (secd.s.x == e.x and secd.s.y == e.y)) and
           ((secd.e.x == arrow.x and secd.e.y == arrow.y) or
            (secd.e.x == e.x and secd.e.y == e.y)) then
            back = true
        end
    end

    local prev = find_arrow_node()
    if not prev then
        prev = #nodes
    end

    if not back then
        if #nodes == 0 then
            new_node(arrow.x, arrow.y)
            prev = 1
        end
        local k = new_node(e.x, e.y)

        new_edge(prev, k, new_bezier({x = arrow.x, y = arrow.y}, e, c, 12), current, c, arrow.step)
    end

    arrow.x, arrow.y = e.x, e.y
    arrow.u, arrow.v = x2n(arrow.x, arrow.y)

    if a < math.pi then
        arrow.a = arrow.a + math.pi/2
    else
        arrow.a = arrow.a - math.pi/2
    end

    --[[
    for i, edge in ipairs(edges) do
        if edge then
            arc_intersects(edge, i)
        end
    end
    ]]

    arc_intersects(edges[#edges], #edges)

    remove_double_edges()

    find_polys()
    update_points()
    count_points()

    if arrow.current == 1 then arrow.current = 2 else arrow.current = 1 end
    arrow.step = arrow.step + 1

    if arrow.u < 1 or arrow.v < 1 or arrow.u > grid.resolution or arrow.v > grid.resolution then
        winner = arrow.current
    end

    if arrow.step >= 12 then
        if player.one.points > player.two.points then
            winner = 1
        else
            winner = 2
        end
    end

    change_time()

    -- debug_edges()
end

function node_on_edge(node, edge)
    local o = false
    
    for i, v in ipairs(edge.path) do
        if o then
            local d = ((v.x - o.x)^2 + (v.y - o.y)^2)^0.5

            for t = 0, 1, 1 / d do
                local m = lerpPQ(v, o, t)
                if (near(m.x, node.x) and near(m.y, node.y)) and not 
                   ((near(m.x, edge.s.x) and near(m.y, edge.s.y)) or
                    (near(m.x, edge.e.x) and near(m.y, edge.e.y))) then
                    return true
                end
            end
        end

        o = v
    end

    return false
end

function remove_double_edges()
    for i, edge in ipairs(edges) do      
        --[[
        for j, node in ipairs(nodes) do
            if node_on_edge(node, edge) then
                print("Node "..j.." on Edge "..i)
                table.remove(edges, i)
            end
        end
        ]]
        
        for j, secd in ipairs(edges) do
            if i ~= j then
                if edge.i == secd.i then
                    for k, third in ipairs(edges) do
                        if k ~= i and k ~= j then
                            if third.i == secd.j and third.j == edge.j and 
                               edge.c.x == secd.c.x and edge.c.y == secd.c.y and 
                               secd.c.x == third.c.x and secd.c.y == third.c.y then
                                print("Removed: "..edge.i.."-"..edge.j..", because "..secd.i.."-"..secd.j.." and "..third.i.."-"..third.j)
                                table.remove(edges, i)
                            end
                        end
                    end
                end
            end
        end
        
    end
end

function debug_edges()
    for i, edge in ipairs(edges) do
        print("Edge "..i..": i = "..edge.i..", j = "..edge.j)
    end
    print()
end

function change_time()
    player.one.time = time
    player.two.time = time
    player.one.check = player.one.time_left
    player.two.check = player.two.time_left
end

function update_players_time()
    if arrow.current == 1 then
        player.one.time_left = player.one.check - (time - player.one.time)
    elseif arrow.current == 2 then
        player.two.time_left = player.two.check - (time - player.two.time)
    end
end

function check_time_won()
    if player.one.time_left <= 0 then
        winner = 2
    elseif player.two.time_left <= 0 then
        winner = 1
    end
end

function draw_endscreen()
    mono = love.graphics.newFont("jetbrains-mono.ttf", 60)
    love.graphics.setFont(mono)

    if winner == 1 then

        love.graphics.setColor(scheme.one)
        love.graphics.printf("Yellow won!", 0, height / 2, width, "center")
    else
        love.graphics.setColor(scheme.two)
        love.graphics.printf("Green won!", 0, height / 2, width, "center")
    end
end

scheme = {
    background = {78/255, 69/255, 92/255},
    white = {1, 1, 1},
    one = {245/255, 226/255, 35/255},
    two = {0/255, 237/255, 79/255},
    node = {245/255, 44/255, 106/255}
}

function love.load()
    width, height = 480, 830
    love.window.setMode(width, height, {msaa = 0})
    love.window.setTitle("Polycentrics")
    love.window.setPosition(1260, 100, 1)

    love.graphics.setBackgroundColor(scheme.background)
    love.graphics.setLineStyle("smooth")

    winner = nil

    time = love.timer.getTime()

    counter = {
        height = 110,
        time = 60
    }

    player = {
        one = {
            time = time,
            check = counter.time,
            points = 0,
            time_left = counter.time
        },

        two = {
            time = time,
            check = counter.time,
            points = 0,
            time_left = counter.time
        },
    }

    nodes = {}
    edges = {}
    polys = {}

    grid = {
        resolution = 11,
        points = {}
    }
    grid.scale = width / 11
    grid.offset = {
        x = - grid.scale / 2,
        y = counter.height - (grid.scale / 2)
    }
    grid.quad_scale = (2*((grid.scale)^2))^0.5

    for v = 1, grid.resolution do
        grid.points[v] = {}

        for u = 1, grid.resolution do
            grid.points[v][u] = 0--math.random(0, 2)
        end
    end

    --count_points()

    frostbite = love.graphics.newFont("frostbite.ttf", 35)
    love.graphics.setFont(frostbite)

    mono = love.graphics.newFont("jetbrains-mono.ttf", 14)

    images = {
        counter = love.graphics.newImage(love.image.newImageData("Counter_Low.png")),
        arrow = love.graphics.newImage("Arrow_Low.png"),

        left_up_x3 = love.graphics.newImage("buttons/Button_Left_Up_X3_Low.png"),
        left_up_x3_pressed = love.graphics.newImage("buttons/Button_Left_Up_X3_Pressed_Low.png"),

        left_up_x2 = love.graphics.newImage("buttons/Button_Left_Up_X2_Low.png"),
        left_up_x2_pressed = love.graphics.newImage("buttons/Button_Left_Up_X2_Pressed_Low.png"),

        left_up_x1 = love.graphics.newImage("buttons/Button_Left_Up_X1_Low.png"),
        left_up_x1_pressed = love.graphics.newImage("buttons/Button_Left_Up_X1_Pressed_Low.png"),

        left_down_x3 = love.graphics.newImage("buttons/Button_Left_Down_X3_Low.png"),
        left_down_x3_pressed = love.graphics.newImage("buttons/Button_Left_Down_X3_Pressed_Low.png"),

        left_down_x2 = love.graphics.newImage("buttons/Button_Left_Down_X2_Low.png"),
        left_down_x2_pressed = love.graphics.newImage("buttons/Button_Left_Down_X2_Pressed_Low.png"),

        left_down_x1 = love.graphics.newImage("buttons/Button_Left_Down_X1_Low.png"),
        left_down_x1_pressed = love.graphics.newImage("buttons/Button_Left_Down_X1_Pressed_Low.png")
    }

    --love.window.setIcon(images.icon)

    arrow = {
        u = 6,
        v = 6,

        a = 0,

        width = grid.scale / 3,
        height = grid.scale / 3,

        image_width = images.arrow:getWidth(),
        image_height = images.arrow:getHeight(),

        current = 2,
        step = 0
    }
    arrow.scale_x = arrow.width / arrow.image_width
    arrow.scale_y = arrow.height / arrow.image_height

    buttons = {}
    button_grid = new_grid(0+3, 590-3, 480-6, 240, 6, 2, 12, true)
    
    local dim = item_dimensions_in_grid(button_grid, 1, 1)
    new_button(function()
        buttons[1].image = images.left_up_x3_pressed
        next_arc(3, 1)
        --buttons[1].call = nil
    end, images.left_up_x3, dim.x, dim.y, dim.width, dim.height)

    local dim = item_dimensions_in_grid(button_grid, 2, 1)
    new_button(function()
        buttons[2].image = images.left_up_x2_pressed
        next_arc(2, 1)
        --buttons[2].call = nil
    end, images.left_up_x2, dim.x, dim.y, dim.width, dim.height)

    local dim = item_dimensions_in_grid(button_grid, 3, 1)
    new_button(function()
        buttons[3].image = images.left_up_x1_pressed
        next_arc(1, 1)
        --buttons[3].call = nil
    end, images.left_up_x1, dim.x, dim.y, dim.width, dim.height)



    local dim = item_dimensions_in_grid(button_grid, 4, 1)
    new_button(function()
        buttons[4].image = images.left_up_x1_pressed
        --buttons[4].call = nil
        next_arc(1, 4)
    end, images.left_up_x1, dim.x, dim.y, dim.width, dim.height, true)

    local dim = item_dimensions_in_grid(button_grid, 5, 1)
    new_button(function()
        buttons[5].image = images.left_up_x2_pressed
        next_arc(2, 4)
        --buttons[5].call = nil
    end, images.left_up_x2, dim.x, dim.y, dim.width, dim.height, true)

    local dim = item_dimensions_in_grid(button_grid, 6, 1)
    new_button(function()
        buttons[6].image = images.left_up_x3_pressed
        next_arc(3, 4)
        --buttons[6].call = nil
    end, images.left_up_x3, dim.x, dim.y, dim.width, dim.height, true)



    local dim = item_dimensions_in_grid(button_grid, 1, 2)
    new_button(function()
        buttons[7].image = images.left_down_x3_pressed
        next_arc(3, 2)
        --buttons[7].call = nil
    end, images.left_down_x3, dim.x, dim.y, dim.width, dim.height)

    local dim = item_dimensions_in_grid(button_grid, 2, 2)
    new_button(function()
        buttons[8].image = images.left_down_x2_pressed
        next_arc(2, 2)
        --buttons[8].call = nil
    end, images.left_down_x2, dim.x, dim.y, dim.width, dim.height)

    local dim = item_dimensions_in_grid(button_grid, 3, 2)
    new_button(function()
        buttons[9].image = images.left_down_x1_pressed
        next_arc(1, 2)
        --buttons[9].call = nil
    end, images.left_down_x1, dim.x, dim.y, dim.width, dim.height)



    local dim = item_dimensions_in_grid(button_grid, 4, 2)
    new_button(function()
        buttons[10].image = images.left_down_x1_pressed
        next_arc(1, 3)
        --buttons[10].call = nil
    end, images.left_down_x1, dim.x, dim.y, dim.width, dim.height, true)

    local dim = item_dimensions_in_grid(button_grid, 5, 2)
    new_button(function()
        buttons[11].image = images.left_down_x2_pressed
        next_arc(2, 3)
        --buttons[11].call = nil
    end, images.left_down_x2, dim.x, dim.y, dim.width, dim.height, true)

    local dim = item_dimensions_in_grid(button_grid, 6, 2)
    new_button(function()
        buttons[12].image = images.left_down_x3_pressed
        next_arc(3, 3)
        --buttons[12].call = nil
    end, images.left_down_x3, dim.x, dim.y, dim.width, dim.height, true)

    --next_arc(2, 3)
    --cut_edge(edges[1], 1, 5, {x = 150, y = 280}, 10, {x = 100, y = 200})
end

function love.mousepressed(mouse_x, mouse_y, mouse_button)
    if winner == nil then
        anker = true

        for i, button in ipairs(buttons) do
            button.over = in_button(mouse_x, mouse_y, button)
        end
    end
end

function love.mousemoved(mouse_x, mouse_y, mouse_button)
    if winner == nil then
        if anker then
            for i, button in ipairs(buttons) do
                button.over = in_button(mouse_x, mouse_y, button)
            end
        end
    end
end

function love.mousereleased(mouse_x, mouse_y, mouse_button)
    if winner == nil then
        for i, button in ipairs(buttons) do
            if in_button(mouse_x, mouse_y, button) and not button.pressed then
                button.call()
                button.pressed = true
            end

            button.over = false
        end

        anker = false
    end
end

function love.update()
    if winner ~= nil then

    else
        time = love.timer.getTime()
        update_players_time()
        check_time_won()
    end
end

function love.draw()
    draw_edges()

    --draw_polys()
    --draw_nodes()

    draw_points()
    draw_arrow()

    draw_grid(button_grid)
    draw_buttons()

    draw_counter()

    if winner ~= nil then
        love.graphics.setColor(scheme.background[1], scheme.background[2], scheme.background[3], 0.75)
        love.graphics.rectangle("fill", 0, 0, width, height)

        draw_endscreen()
    end
end