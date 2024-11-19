#version 450

layout(location = 0) in vec3 pos;
layout(location = 1) in vec4 color;
layout(location = 2) in vec2 cords;

layout(location = 0) out vec2 TexCoords;
layout(location = 1) out vec4 VertexColor;

void main()
{
    gl_Position = vec4(pos, 1.0);
    TexCoords = cords;
    VertexColor = color;
}