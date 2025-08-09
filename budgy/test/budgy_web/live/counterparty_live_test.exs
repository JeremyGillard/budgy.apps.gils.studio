defmodule BudgyWeb.CounterpartyLiveTest do
  use BudgyWeb.ConnCase

  import Phoenix.LiveViewTest
  import Budgy.BankingFixtures

  @create_attrs %{name: "some name", account: "some account"}
  @update_attrs %{name: "some updated name", account: "some updated account"}
  @invalid_attrs %{name: nil, account: nil}
  defp create_counterparty(_) do
    counterparty = counterparty_fixture()

    %{counterparty: counterparty}
  end

  describe "Index" do
    setup [:create_counterparty]

    test "lists all counterparties", %{conn: conn, counterparty: counterparty} do
      {:ok, _index_live, html} = live(conn, ~p"/counterparties")

      assert html =~ "Listing Counterparties"
      assert html =~ counterparty.name
    end

    test "saves new counterparty", %{conn: conn} do
      {:ok, index_live, _html} = live(conn, ~p"/counterparties")

      assert {:ok, form_live, _} =
               index_live
               |> element("a", "New Counterparty")
               |> render_click()
               |> follow_redirect(conn, ~p"/counterparties/new")

      assert render(form_live) =~ "New Counterparty"

      assert form_live
             |> form("#counterparty-form", counterparty: @invalid_attrs)
             |> render_change() =~ "can&#39;t be blank"

      assert {:ok, index_live, _html} =
               form_live
               |> form("#counterparty-form", counterparty: @create_attrs)
               |> render_submit()
               |> follow_redirect(conn, ~p"/counterparties")

      html = render(index_live)
      assert html =~ "Counterparty created successfully"
      assert html =~ "some name"
    end

    test "updates counterparty in listing", %{conn: conn, counterparty: counterparty} do
      {:ok, index_live, _html} = live(conn, ~p"/counterparties")

      assert {:ok, form_live, _html} =
               index_live
               |> element("#counterparties-#{counterparty.id} a", "Edit")
               |> render_click()
               |> follow_redirect(conn, ~p"/counterparties/#{counterparty}/edit")

      assert render(form_live) =~ "Edit Counterparty"

      assert form_live
             |> form("#counterparty-form", counterparty: @invalid_attrs)
             |> render_change() =~ "can&#39;t be blank"

      assert {:ok, index_live, _html} =
               form_live
               |> form("#counterparty-form", counterparty: @update_attrs)
               |> render_submit()
               |> follow_redirect(conn, ~p"/counterparties")

      html = render(index_live)
      assert html =~ "Counterparty updated successfully"
      assert html =~ "some updated name"
    end

    test "deletes counterparty in listing", %{conn: conn, counterparty: counterparty} do
      {:ok, index_live, _html} = live(conn, ~p"/counterparties")

      assert index_live |> element("#counterparties-#{counterparty.id} a", "Delete") |> render_click()
      refute has_element?(index_live, "#counterparties-#{counterparty.id}")
    end
  end

  describe "Show" do
    setup [:create_counterparty]

    test "displays counterparty", %{conn: conn, counterparty: counterparty} do
      {:ok, _show_live, html} = live(conn, ~p"/counterparties/#{counterparty}")

      assert html =~ "Show Counterparty"
      assert html =~ counterparty.name
    end

    test "updates counterparty and returns to show", %{conn: conn, counterparty: counterparty} do
      {:ok, show_live, _html} = live(conn, ~p"/counterparties/#{counterparty}")

      assert {:ok, form_live, _} =
               show_live
               |> element("a", "Edit")
               |> render_click()
               |> follow_redirect(conn, ~p"/counterparties/#{counterparty}/edit?return_to=show")

      assert render(form_live) =~ "Edit Counterparty"

      assert form_live
             |> form("#counterparty-form", counterparty: @invalid_attrs)
             |> render_change() =~ "can&#39;t be blank"

      assert {:ok, show_live, _html} =
               form_live
               |> form("#counterparty-form", counterparty: @update_attrs)
               |> render_submit()
               |> follow_redirect(conn, ~p"/counterparties/#{counterparty}")

      html = render(show_live)
      assert html =~ "Counterparty updated successfully"
      assert html =~ "some updated name"
    end
  end
end
