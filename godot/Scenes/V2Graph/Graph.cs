using System.Linq;
using Godot;

namespace GCCC.Scenes.V2Graph;

/// <summary>
/// Represents a graph visualization node in the scene tree
/// </summary>
public partial class Graph : Node
{
    [Export]
    private int _Resolution = 100;

    private float Step => 2.0f / _Resolution;

    private Vector3 Size => Vector3.One * Step;

    private readonly StandardMaterial3D _material = new()
    {
        AlbedoColor = Colors.Red
    };

    // Called when the node enters the scene tree for the first time.
    public override void _Ready()
    {
        for (int i = 0; i < _Resolution * _Resolution; i++)
        {
            AddChild(new CsgBox3D
            {
                Size = Size,
                Material = _material
            });
        }
    }

    // Called every frame. 'delta' is the elapsed time since the previous frame.
    public override void _Process(double delta)
    {
        var time = Time.GetTicksMsec() / 1000.0f;

        for (var i = 0; i < GetChildCount(); i++)
        {
            if (GetChild(i) is not CsgBox3D point) continue;

            var (u, v) = (i % _Resolution, i / _Resolution);
            var (x, z) = (
                u * 2.0f / _Resolution - 1.0f,
                v * 2.0f / _Resolution - 1.0f 
            );

            point.Position = new(
                x,
                2.0f * Mathf.Sin(time + x + z),
                z
            );
        }
    }
}
