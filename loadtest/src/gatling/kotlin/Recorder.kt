import io.gatling.recorder.GatlingRecorder
import io.gatling.recorder.config.RecorderPropertiesBuilder
import scala.Option

object Recorder {

  @JvmStatic
  fun main(args: Array<String>) {
    GatlingRecorder.fromMap(
        RecorderPropertiesBuilder()
            .simulationsFolder(IDEPathHelper.gradleSourcesDirectory.toString())
            .resourcesFolder(IDEPathHelper.gradleResourcesDirectory.toString())
            .simulationPackage("simulations")
            .build(),
        Option.apply(IDEPathHelper.recorderConfigFile))
  }
}
