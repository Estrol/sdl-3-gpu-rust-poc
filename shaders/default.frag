#version 450

layout(location = 0) in vec2 TexCoords;
layout(location = 1) in vec4 VertexColor;

layout(location = 0) out vec4 FragColor;

layout(set = 2, binding = 0) uniform sampler2D texture1;

void main()
{
    FragColor = texture(texture1, TexCoords) * VertexColor;
}