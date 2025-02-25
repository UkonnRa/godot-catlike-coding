using Godot;
using System;

namespace GCCC.Scenes.V2Graph;

public partial class FPSLabel : Label
{
    public override void _Process(double delta)
    {
        Text = $"{Engine.GetFramesPerSecond()} FPS - {delta:N2} ms";
    }
}
