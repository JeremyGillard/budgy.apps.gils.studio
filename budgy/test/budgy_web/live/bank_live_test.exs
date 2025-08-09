defmodule BudgyWeb.BankLiveTest do
  use BudgyWeb.ConnCase

  import Phoenix.LiveViewTest
  import Budgy.BankingFixtures

  @create_attrs %{name: "some name", bic: "some bic", country_code: "some country_code"}
  @update_attrs %{name: "some updated name", bic: "some updated bic", country_code: "some updated country_code"}
  @invalid_attrs %{name: nil, bic: nil, country_code: nil}
  defp create_bank(_) do
    bank = bank_fixture()

    %{bank: bank}
  end

  describe "Index" do
    setup [:create_bank]

    test "lists all banks", %{conn: conn, bank: bank} do
      {:ok, _index_live, html} = live(conn, ~p"/banks")

      assert html =~ "Listing Banks"
      assert html =~ bank.name
    end

    test "saves new bank", %{conn: conn} do
      {:ok, index_live, _html} = live(conn, ~p"/banks")

      assert {:ok, form_live, _} =
               index_live
               |> element("a", "New Bank")
               |> render_click()
               |> follow_redirect(conn, ~p"/banks/new")

      assert render(form_live) =~ "New Bank"

      assert form_live
             |> form("#bank-form", bank: @invalid_attrs)
             |> render_change() =~ "can&#39;t be blank"

      assert {:ok, index_live, _html} =
               form_live
               |> form("#bank-form", bank: @create_attrs)
               |> render_submit()
               |> follow_redirect(conn, ~p"/banks")

      html = render(index_live)
      assert html =~ "Bank created successfully"
      assert html =~ "some name"
    end

    test "updates bank in listing", %{conn: conn, bank: bank} do
      {:ok, index_live, _html} = live(conn, ~p"/banks")

      assert {:ok, form_live, _html} =
               index_live
               |> element("#banks-#{bank.id} a", "Edit")
               |> render_click()
               |> follow_redirect(conn, ~p"/banks/#{bank}/edit")

      assert render(form_live) =~ "Edit Bank"

      assert form_live
             |> form("#bank-form", bank: @invalid_attrs)
             |> render_change() =~ "can&#39;t be blank"

      assert {:ok, index_live, _html} =
               form_live
               |> form("#bank-form", bank: @update_attrs)
               |> render_submit()
               |> follow_redirect(conn, ~p"/banks")

      html = render(index_live)
      assert html =~ "Bank updated successfully"
      assert html =~ "some updated name"
    end

    test "deletes bank in listing", %{conn: conn, bank: bank} do
      {:ok, index_live, _html} = live(conn, ~p"/banks")

      assert index_live |> element("#banks-#{bank.id} a", "Delete") |> render_click()
      refute has_element?(index_live, "#banks-#{bank.id}")
    end
  end

  describe "Show" do
    setup [:create_bank]

    test "displays bank", %{conn: conn, bank: bank} do
      {:ok, _show_live, html} = live(conn, ~p"/banks/#{bank}")

      assert html =~ "Show Bank"
      assert html =~ bank.name
    end

    test "updates bank and returns to show", %{conn: conn, bank: bank} do
      {:ok, show_live, _html} = live(conn, ~p"/banks/#{bank}")

      assert {:ok, form_live, _} =
               show_live
               |> element("a", "Edit")
               |> render_click()
               |> follow_redirect(conn, ~p"/banks/#{bank}/edit?return_to=show")

      assert render(form_live) =~ "Edit Bank"

      assert form_live
             |> form("#bank-form", bank: @invalid_attrs)
             |> render_change() =~ "can&#39;t be blank"

      assert {:ok, show_live, _html} =
               form_live
               |> form("#bank-form", bank: @update_attrs)
               |> render_submit()
               |> follow_redirect(conn, ~p"/banks/#{bank}")

      html = render(show_live)
      assert html =~ "Bank updated successfully"
      assert html =~ "some updated name"
    end
  end
end
