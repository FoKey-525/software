import React from "react"

function App() {
  return (
    <>
      <div className="row row-cols-1 row-cols-sm-2 row-cols-md-3 g-3 m-3">
        <div className="card">
         <h3 className="card-text">Биология</h3>

          <div className="menu-button d-flex">
            <form method="post" action=""><input className="m-1 btn btn-primary rounded-pill px-3" type="submit" value="Сохранить обои" /></form>
            <form method="post" action=""><input className="m-1 btn btn-primary rounded-pill px-3" type="submit" value="Сбросить обои" /></form>
            <form method="post" action=""><input className="m-1 btn btn-primary rounded-pill px-3" type="submit" value="Поменять обои" /></form>
          </div>
        </div>

        <div className="card">
         <h3 className="card-text">Технология</h3>

          <div className="menu-button d-flex">
            <form method="post" action=""><input className="m-1 btn btn-primary rounded-pill px-3" type="submit" value="Сохранить обои" /></form>
            <form method="post" action=""><input className="m-1 btn btn-primary rounded-pill px-3" type="submit" value="Сбросить обои" /></form>
            <form method="post" action=""><input className="m-1 btn btn-primary rounded-pill px-3" type="submit" value="Поменять обои" /></form>
          </div>
        </div>

        <div className="card">
         <h3 className="card-text">Физика</h3>

          <div className="menu-button d-flex">
            <form method="post" action=""><input className="m-1 btn btn-primary rounded-pill px-3" type="submit" value="Сохранить обои" /></form>
            <form method="post" action=""><input className="m-1 btn btn-primary rounded-pill px-3" type="submit" value="Сбросить обои" /></form>
            <form method="post" action=""><input className="m-1 btn btn-primary rounded-pill px-3" type="submit" value="Поменять обои" /></form>
          </div>
        </div>
      </div>
    </>
  )
}

export default App
