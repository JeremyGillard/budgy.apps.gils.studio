defmodule BudgyWeb.PageController do
  use BudgyWeb, :controller

  def home(conn, _params) do
    render(conn, :home)
  end
end
