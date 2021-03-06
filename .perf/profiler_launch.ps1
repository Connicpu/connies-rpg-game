Enable-VSPrompt 14

if (Test-Path ".perf\perfreport.vspx") {
    Remove-Item ".perf\perfreport.vspx"
}

try
{
    Start-Process VSPerf -Verb RunAs -ArgumentList "/launch:target\release\connies-rpg-game.exe /file:.perf\perfreport.vspx"

    while (!(Test-Path ".perf\perfreport.vspx")) {
        Start-Sleep -Seconds 1
    }

    Start-Process devenv.exe .perf\perfreport.vspx
}
catch {}
