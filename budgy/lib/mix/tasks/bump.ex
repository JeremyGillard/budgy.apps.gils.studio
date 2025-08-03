defmodule Mix.Tasks.Bump do
  use Mix.Task

  @shortdoc "Bumps the version number"
  @moduledoc """
  Bumps the version number in your project.

  ## Usage

      mix bump [major|minor|patch]

  With no argument, defaults to patch bump.
  """

  def run(args) do
    bump_type = parse_bump(args)
    old_version = Mix.Project.config()[:version]
    new_version = bump(old_version, bump_type)

    IO.puts("Bumping version from #{old_version} to #{new_version}")

    if git_available?() do
      update_mix_exs(old_version, new_version)
      create_git_tag(new_version)
    else
      IO.puts("⚠️ Git not available. Skipping tag creation.")
    end
  end

  defp parse_bump(["major"]), do: :major
  defp parse_bump(["minor"]), do: :minor
  defp parse_bump(["patch"]), do: :patch
  defp parse_bump(_), do: :patch

  defp bump(version, type) do
    [maj, min, patch] = version |> String.split(".") |> Enum.map(&String.to_integer/1)

    case type do
      :major -> "#{maj + 1}.0.0"
      :minor -> "#{maj}.#{min + 1}.0"
      :patch -> "#{maj}.#{min}.#{patch + 1}"
    end
  end

  defp update_mix_exs(old_version, new_version) do
    path = "mix.exs"
    content = File.read!(path)

    updated =
      Regex.replace(~r/version: "#{Regex.escape(old_version)}"/, content, "version: \"#{new_version}\"")

    File.write!(path, updated)
    git_amend(path)
    IO.puts("mix.exs updated successfully.")
  end

  defp git_available? do
    System.find_executable("git") != nil
  end

  defp create_git_tag(version) do
    tag = "v#{version}"
    message = "Release #{tag}"

    {_, status} = System.cmd("git", ["tag", "-a", tag, "-m", message])
    if status == 0 do
      IO.puts("🏷️ Git tag #{tag} created.")
    else
      IO.puts("❌ Failed to create Git tag.")
    end
  end

  defp git_amend(file_path) do
    {_, add_status} = System.cmd("git", ["add", file_path])

    if add_status == 0 do
      {_, amend_status} = System.cmd("git", ["commit", "--amend", "--no-edit"])

      if amend_status == 0 do
        IO.puts("📝 Last commit amended to include #{file_path}.")
      else
        IO.puts("❌ Failed to amend the commit.")
      end
    else
      IO.puts("❌ Failed to stage #{file_path}.")
    end
  end
end
