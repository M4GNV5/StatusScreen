//
//config
//
thickness = 1;
size = [170, 100, 30];
frontHeight = 5;
angle = 30;



//
//code
//
sizeDiff = size[2] / tan(90 - angle);

module toWall(points, offset)
{

	points2 = [for(p = points) [
		p[0] + offset[0],
		p[1] + offset[1],
		p[2] + offset[2],
	]];

	count = len(points);
	face1 = [for(i = [0 : count - 1]) i];
	face2 = [for(i = [0 : count - 1]) i + count];
	faces = [for(i = [0 : count - 1]) i == 0
				? [count - 1, 0, count, count + count - 1]
				: [i - 1, i, i + count, i + count - 1]
			];

	polyhedron(concat(points, points2), concat([face1], [face2], faces));
}

module drawSide(thick)
{
	toWall([
		[0, -size[1] / 2, 0],
		[0, size[1] / 2, 0],
		[0, size[1] / 2, size[2]],
		[0, -size[1] / 2 + sizeDiff, size[2]],
	],
	[thick, 0, 0]);
}

//floor
translate([0, 0, thickness / 2])
	cube([size[0], size[1], thickness], true);

//roof
translate([0, sizeDiff / 2, size[2] - thickness / 2])
	cube([size[0], size[1] - sizeDiff, thickness], true);

//front
translate([0, -size[1] / 2, frontHeight / 2])
	cube([size[0], thickness, frontHeight], true);

//side X+
translate([size[0] / 2, 0, 0])
	drawSide(-thickness);

//side X-
translate([-size[0] / 2, 0, 0])
	drawSide(thickness);
