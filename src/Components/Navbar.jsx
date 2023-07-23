import ProcessSVG from "../assets/processSVG.svg";
import MemorySVG from "../assets/memory.svg";
import "./Navbar.css";
export default function Navbar() {
  return (
    <>
      <div className="sidenav">
        <div className="sidenav-nav">
          <a  className="sidenav-a"href="#">
            <img
              src={ProcessSVG}
              style={{ height: 35, width: 35 }}
              alt="Process"
            />
            <span>Process</span>
          </a>

          <a href="#">
           
            <img src={MemorySVG} style={{ height: 35, width: 35 }} alt="mem" />
            <span>Monitor</span>
          </a>

          {/* <a href="#">
            <span>About</span>
          </a>
          <a href="#">
            <span>Contact</span>
          </a> */}
        </div>
      </div>
    </>
  );
}
